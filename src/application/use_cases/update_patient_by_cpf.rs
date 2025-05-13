use crate::{
    application::errors::patient_application_error::PatientApplicationError,
    domain::{entities::patient::Patient, repositories::patient_repository::PatientRepository},
    presentation::dtos::patient_dto::UpdatePatientDTO,
};

pub struct UpdatePatientByCpfUseCase<T: PatientRepository> {
    patient_repo: T,
}

impl<T: PatientRepository> UpdatePatientByCpfUseCase<T> {
    pub fn new(patient_repo: T) -> Self {
        Self { patient_repo }
    }

    pub async fn execute(
        &self,
        cpf: String,
        updated_patient: UpdatePatientDTO,
    ) -> Result<Patient, PatientApplicationError> {
        let patient = self.patient_repo.find_by_cpf(cpf.clone()).await?;

        if patient.is_none() {
            return Err(PatientApplicationError::NotFound(cpf));
        }

        let mut patient = patient.unwrap();
        patient.name = updated_patient.name.unwrap_or(patient.name);

        self.patient_repo
            .update(&patient)
            .await
            .map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;

    use crate::{
        application::{
            errors::patient_application_error::PatientApplicationError,
            use_cases::update_patient_by_cpf::UpdatePatientByCpfUseCase,
        },
        domain::{
            entities::patient::Patient, errors::repository_error::RepositoryError,
            repositories::patient_repository::MockPatientRepository,
        },
        presentation::dtos::patient_dto::UpdatePatientDTO,
    };

    #[tokio::test]
    async fn execute_patient_repository_find_by_cpf_error() {
        let mut mock_patient_repo = MockPatientRepository::new();
        let (cpf, updated_patient) = make_fake_input(None);

        mock_patient_repo
            .expect_find_by_cpf()
            .times(1)
            .return_const(Err(RepositoryError::DatabaseError(
                "Fake Error".to_string(),
            )));

        let sut = UpdatePatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute(cpf, updated_patient).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_patient_not_found() {
        let mut mock_patient_repo = MockPatientRepository::new();
        let (cpf, updated_patient) = make_fake_input(None);

        mock_patient_repo
            .expect_find_by_cpf()
            .times(1)
            .return_const(Ok(None));

        let sut = UpdatePatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute(cpf.clone(), updated_patient).await;

        assert_eq!(result, Err(PatientApplicationError::NotFound(cpf)));
    }

    #[tokio::test]
    async fn execute_patient_repository_update_error() {
        let mut mock_patient_repo = MockPatientRepository::new();
        let (cpf, updated_patient) = make_fake_input(None);

        mock_patient_repo
            .expect_find_by_cpf()
            .times(1)
            .return_const(Ok(Some(make_fake_patient())));

        mock_patient_repo.expect_update().times(1).return_const(Err(
            RepositoryError::DatabaseError("Fake Error".to_string()),
        ));

        let sut = UpdatePatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute(cpf.clone(), updated_patient).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_ok() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_patient_repo = MockPatientRepository::new();
        let new_name = "Andrew Monteiro".to_string();
        let (cpf, updated_patient) = make_fake_input(Some(new_name.clone()));
        let fake_patient = make_fake_patient();
        let mut updated_patient_entity = fake_patient.clone();
        updated_patient_entity.name = new_name.clone();

        mock_patient_repo
            .expect_find_by_cpf()
            .times(1)
            .with(eq(cpf.clone()))
            .return_const(Ok(Some(fake_patient.clone())));

        mock_patient_repo
            .expect_update()
            .times(1)
            .with(eq(updated_patient_entity.clone()))
            .return_const(Ok(updated_patient_entity.clone()));

        let sut = UpdatePatientByCpfUseCase::new(mock_patient_repo);

        let result = sut.execute(cpf.clone(), updated_patient).await?;

        assert_eq!(result, updated_patient_entity);

        Ok(())
    }

    fn make_fake_patient() -> Patient {
        Patient::restore(42, "Andrew".to_string(), "12345678901".to_string()).unwrap()
    }

    fn make_fake_input(name: Option<String>) -> (String, UpdatePatientDTO) {
        let cpf = "12345678901".to_string();

        match name {
            Some(name) => (cpf.clone(), UpdatePatientDTO { name: Some(name) }),
            None => (
                cpf.clone(),
                UpdatePatientDTO {
                    name: Some("Andrew Updated".to_string()),
                },
            ),
        }
    }
}
