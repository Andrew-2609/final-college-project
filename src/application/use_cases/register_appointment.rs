use crate::{
    application::errors::appointment_application_error::AppointmentApplicationError,
    domain::{
        entities::appointment::Appointment,
        repositories::{
            appointment_repository::AppointmentRepository, patient_repository::PatientRepository,
        },
    },
    presentation::dtos::appointment_dto::CreateAppointmentDTO,
};

pub struct RegisterAppointmentUseCase<T: AppointmentRepository, P: PatientRepository> {
    appointment_repository: T,
    patient_repository: P,
}

impl<T: AppointmentRepository, P: PatientRepository> RegisterAppointmentUseCase<T, P> {
    pub fn new(appointment_repository: T, patient_repository: P) -> Self {
        Self {
            appointment_repository,
            patient_repository,
        }
    }

    pub async fn execute(
        &self,
        appointment: CreateAppointmentDTO,
    ) -> Result<Appointment, AppointmentApplicationError> {
        let appointment = Appointment::try_from(appointment)?;

        let patient = self
            .patient_repository
            .find_by_id(appointment.patient_id)
            .await?;

        if patient.is_none() {
            return Err(AppointmentApplicationError::PatientNotFound(
                appointment.patient_id,
            ));
        }

        let appointment = self.appointment_repository.save(&appointment).await?;

        Ok(appointment)
    }
}
