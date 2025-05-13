use crate::{
    application::errors::patient_application_error::PatientApplicationError,
    domain::{entities::patient::Patient, repositories::patient_repository::PatientRepository},
    presentation::dtos::patient_dto::CreatePatientDTO,
};

pub struct RegisterPatientUseCase<T: PatientRepository> {
    patient_repo: T,
}

impl<T: PatientRepository> RegisterPatientUseCase<T> {
    pub fn new(patient_repo: T) -> Self {
        Self { patient_repo }
    }

    pub async fn execute(&self, patient: CreatePatientDTO) -> Result<i32, PatientApplicationError> {
        if self.patient_repo.exists_by_cpf(&patient.cpf).await? {
            return Err(PatientApplicationError::Conflict(format!(
                "The CPF {} is already taken",
                patient.cpf
            )));
        }

        let patient: Patient = patient.into();

        self.patient_repo
            .save(&patient)
            .await
            .map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::{
            errors::patient_application_error::PatientApplicationError,
            use_cases::register_patient::RegisterPatientUseCase,
        },
        domain::{
            entities::patient::Patient, errors::repository_error::RepositoryError,
            repositories::patient_repository::MockPatientRepository,
        },
        presentation::dtos::patient_dto::CreatePatientDTO,
    };

    #[tokio::test]
    async fn execute_patient_repository_exists_by_cpf_error() {
        let mut mock_patient_repo = MockPatientRepository::new();

        let fake_patient = make_fake_create_patient_dto(None);

        mock_patient_repo
            .expect_exists_by_cpf()
            .times(1)
            .return_const(Err(RepositoryError::DatabaseError(
                "Fake Error".to_string(),
            )));

        mock_patient_repo.expect_save().times(0);

        let sut = RegisterPatientUseCase::new(mock_patient_repo);

        let result = sut.execute(fake_patient.clone()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_cpf_taken_error() {
        let mut mock_patient_repo = MockPatientRepository::new();

        let fake_patient = make_fake_create_patient_dto(None);

        mock_patient_repo
            .expect_exists_by_cpf()
            .times(1)
            .return_const(Ok(true));

        mock_patient_repo.expect_save().times(0);

        let sut = RegisterPatientUseCase::new(mock_patient_repo);

        let result = sut.execute(fake_patient.clone()).await;

        assert_eq!(
            result,
            Err(PatientApplicationError::Conflict(format!(
                "The CPF {} is already taken",
                fake_patient.cpf
            )))
        )
    }

    #[tokio::test]
    async fn execute_patient_repository_save_error() {
        let mut mock_patient_repo = MockPatientRepository::new();

        let fake_patient = make_fake_create_patient_dto(None);

        mock_patient_repo
            .expect_exists_by_cpf()
            .times(1)
            .return_const(Ok(false));

        mock_patient_repo
            .expect_save()
            .times(1)
            .return_const(Err(RepositoryError::DatabaseError(
                "Fake Error".to_string(),
            )));

        let sut = RegisterPatientUseCase::new(mock_patient_repo);

        let result = sut.execute(fake_patient.clone()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_ok() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_patient_repo = MockPatientRepository::new();

        let cpf = "11122233344";
        let fake_patient = make_fake_create_patient_dto(Some(cpf.to_string()));

        let fake_patient_entity: Patient = fake_patient.clone().into();

        mock_patient_repo
            .expect_exists_by_cpf()
            .withf(|expected_email: &str| *expected_email == *cpf)
            .times(1)
            .return_const(Ok(false));

        let new_patient_id = 42;

        mock_patient_repo
            .expect_save()
            .withf(move |expected_patient: &Patient| *expected_patient == fake_patient_entity)
            .times(1)
            .return_const(Ok(new_patient_id));

        let sut = RegisterPatientUseCase::new(mock_patient_repo);

        let result = sut.execute(fake_patient.clone()).await?;

        assert_eq!(result, new_patient_id);

        Ok(())
    }

    fn make_fake_create_patient_dto(cpf: Option<String>) -> CreatePatientDTO {
        CreatePatientDTO {
            name: "Andrew".to_string(),
            cpf: match cpf {
                None => "00011122233".to_string(),
                Some(value) => value,
            },
        }
    }
}
