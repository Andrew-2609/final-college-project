use crate::{
    application::errors::appointment_application_error::AppointmentApplicationError,
    domain::{
        entities::appointment::Appointment,
        repositories::{
            appointment_repository::AppointmentRepository, patient_repository::PatientRepository,
        },
    },
};

pub struct ListAppointmentsByPatientCpfUseCase<T: PatientRepository, P: AppointmentRepository> {
    patient_repo: T,
    appointment_repo: P,
}

impl<T: PatientRepository, P: AppointmentRepository> ListAppointmentsByPatientCpfUseCase<T, P> {
    pub fn new(patient_repo: T, appointment_repo: P) -> Self {
        Self {
            patient_repo,
            appointment_repo,
        }
    }

    pub async fn execute(
        &self,
        cpf: String,
    ) -> Result<Vec<Appointment>, AppointmentApplicationError> {
        let patient = self.patient_repo.find_by_cpf(cpf.clone()).await?;

        if patient.is_none() {
            return Err(AppointmentApplicationError::PatientNotFound(cpf));
        }

        let patient = patient.unwrap();

        if !patient.id.is_existing() {
            return Err(AppointmentApplicationError::PatientNotFound(cpf));
        }

        let patient_id: Option<i32> = patient.id.into();
        let patient_id = patient_id.unwrap_or(0);

        self.appointment_repo
            .find_by_patient_id(patient_id)
            .await
            .map_err(|err| err.into())
    }
}
