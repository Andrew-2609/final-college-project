use chrono::NaiveDateTime;

use crate::{
    application::errors::appointment_application_error::AppointmentApplicationError,
    domain::{
        entities::appointment::Appointment,
        repositories::{
            appointment_repository::AppointmentRepository, patient_repository::PatientRepository,
        },
    },
    presentation::dtos::appointment_dto::BookAppointmentDTO,
};

pub struct BookAppointmentUseCase<T: AppointmentRepository, P: PatientRepository> {
    appointment_repository: T,
    patient_repository: P,
}

impl<T: AppointmentRepository, P: PatientRepository> BookAppointmentUseCase<T, P> {
    pub fn new(appointment_repository: T, patient_repository: P) -> Self {
        Self {
            appointment_repository,
            patient_repository,
        }
    }

    pub async fn execute(
        &self,
        appointment: BookAppointmentDTO,
    ) -> Result<Appointment, AppointmentApplicationError> {
        let patient = self
            .patient_repository
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

        let appointment = Appointment::new(
            patient_id,
            appointment.appointment_at.parse::<NaiveDateTime>()?,
            appointment.specialty,
            appointment.notes,
        )?;

        if self
            .appointment_repository
            .exists_by_patient_id_and_appointment_at(patient_id, appointment.appointment_at)
            .await?
        {
            return Err(AppointmentApplicationError::Constraint(format!(
                "There's already an appointment for patient with CPF: {} at: {}",
                patient.cpf, appointment.appointment_at
            )));
        }

        let appointment = self.appointment_repository.save(&appointment).await?;

        Ok(appointment)
    }
}
