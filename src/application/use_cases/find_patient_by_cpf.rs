use crate::{
    application::errors::patient_application_error::PatientApplicationError,
    domain::{entities::patient::Patient, repositories::patient_repository::PatientRepository},
};

pub struct FindPatientByCpfUseCase<T: PatientRepository> {
    patient_repo: T,
}

impl<T: PatientRepository> FindPatientByCpfUseCase<T> {
    pub fn new(patient_repo: T) -> Self {
        Self { patient_repo }
    }

    pub async fn execute(&self, cpf: String) -> Result<Option<Patient>, PatientApplicationError> {
        self.patient_repo
            .find_by_cpf(cpf)
            .await
            .map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;

    use crate::{
        application::use_cases::find_patient_by_cpf::FindPatientByCpfUseCase,
        domain::{
            entities::patient::Patient, errors::repository_error::RepositoryError,
            repositories::patient_repository::MockPatientRepository,
        },
    };

    #[tokio::test]
    async fn execute_patient_repository_error() {
        let mut mock_patient_repo = MockPatientRepository::new();

        mock_patient_repo
            .expect_find_by_cpf()
            .times(1)
            .return_const(Err(RepositoryError::DatabaseError(
                "Fake Error".to_string(),
            )));

        let sut = FindPatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute("00011122233".to_string()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_ok() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_patient_repo = MockPatientRepository::new();

        let fake_patient = Patient::restore(42, "Andrew".to_string(), "00011122233".to_string())?;

        mock_patient_repo
            .expect_find_by_cpf()
            .with(eq(fake_patient.cpf.clone()))
            .times(1)
            .return_const(Ok(Some(fake_patient.clone())));

        let sut = FindPatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute(fake_patient.cpf.clone()).await?;

        assert_eq!(result, Some(fake_patient));

        Ok(())
    }
}
