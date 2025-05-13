use crate::{
    application::errors::patient_application_error::PatientApplicationError,
    domain::repositories::patient_repository::PatientRepository,
};

pub struct DeletePatientByCpfUseCase<T: PatientRepository> {
    patient_repo: T,
}

impl<T: PatientRepository> DeletePatientByCpfUseCase<T> {
    pub fn new(patient_repo: T) -> Self {
        Self { patient_repo }
    }

    pub async fn execute(&self, cpf: String) -> Result<(), PatientApplicationError> {
        self.patient_repo
            .delete_by_cpf(cpf)
            .await
            .map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;

    use crate::{
        application::use_cases::delete_patient_by_cpf::DeletePatientByCpfUseCase,
        domain::{
            errors::repository_error::RepositoryError,
            repositories::patient_repository::MockPatientRepository,
        },
    };

    #[tokio::test]
    async fn execute_patient_repository_delete_by_cpf_error() {
        let mut mock_patient_repo = MockPatientRepository::new();

        mock_patient_repo
            .expect_delete_by_cpf()
            .times(1)
            .return_const(Err(RepositoryError::DatabaseError(
                "Fake Error".to_string(),
            )));

        let sut = DeletePatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute("12345678901".to_string()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_ok() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_patient_repo = MockPatientRepository::new();
        let cpf = "12345678901";

        mock_patient_repo
            .expect_delete_by_cpf()
            .times(1)
            .with(eq(cpf.to_string()))
            .return_const(Ok(()));

        let sut = DeletePatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute(cpf.to_string()).await?;

        assert_eq!(result, ());

        Ok(())
    }
}
