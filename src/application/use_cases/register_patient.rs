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
            return Err(PatientApplicationError::CPFAlreadyTaken(patient.cpf));
        }

        let patient: Patient = patient.into();

        self.patient_repo
            .save(&patient)
            .await
            .map_err(|err| err.into())
    }
}
