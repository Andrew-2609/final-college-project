use chrono::NaiveDateTime;

use crate::{
    application::errors::appointment_application_error::AppointmentApplicationError,
    domain::{
        entities::appointment::Appointment,
        repositories::{
            appointment_repository::AppointmentRepository, patient_repository::PatientRepository,
        },
    },
    presentation::dtos::appointment_dto::CancelAppointmentDTO,
};

pub struct CancelAppointmentUseCase<T: AppointmentRepository, P: PatientRepository> {
    appointment_repo: T,
    patient_repo: P,
}

impl<T: AppointmentRepository, P: PatientRepository> CancelAppointmentUseCase<T, P> {
    pub fn new(appointment_repository: T, patient_repository: P) -> Self {
        Self {
            appointment_repo: appointment_repository,
            patient_repo: patient_repository,
        }
    }

    pub async fn execute(
        &self,
        appointment: CancelAppointmentDTO,
    ) -> Result<Appointment, AppointmentApplicationError> {
        let patient = self
            .patient_repo
            .find_by_cpf(appointment.patient_cpf.clone())
            .await?;

        if patient.is_none() {
            return Err(AppointmentApplicationError::PatientNotFound(
                appointment.patient_cpf,
            ));
        }

        let patient = patient.unwrap();

        if !patient.id.is_existing() {
            return Err(AppointmentApplicationError::PatientNotFound(
                appointment.patient_cpf,
            ));
        }

        let patient_id: Option<i32> = patient.id.into();
        let patient_id = patient_id.unwrap_or(0);
        let cancellation_reason = appointment.cancellation_reason.unwrap_or_default();
        let appointment_at = appointment.appointment_at.parse::<NaiveDateTime>()?;

        let appointment = self
            .appointment_repo
            .find_by_patient_id_and_appointment_at(patient_id, appointment_at)
            .await?;

        if appointment.is_none() {
            return Err(AppointmentApplicationError::NotFound(format!(
                "No appointment found for patient with CPF: {} at: {}",
                patient.cpf, appointment_at
            )));
        }

        let mut appointment = appointment.unwrap();
        if appointment.is_canceled() {
            return Ok(appointment);
        }
        appointment.cancel(cancellation_reason);

        self.appointment_repo
            .update(&appointment)
            .await
            .map_err(|err| err.into())
    }
}
