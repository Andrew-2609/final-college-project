use async_trait::async_trait;
use chrono::NaiveDateTime;

use crate::domain::{
    entities::appointment::Appointment, errors::repository_error::RepositoryError,
};

#[async_trait]
pub trait AppointmentRepository {
    async fn exists_by_patient_id_and_appointment_at(
        &self,
        patient_id: i32,
        appointment_at: NaiveDateTime,
    ) -> Result<bool, RepositoryError>;
    async fn save(&self, appointment: &Appointment) -> Result<Appointment, RepositoryError>;
    async fn find_by_patient_id_and_appointment_at(
        &self,
        patient_id: i32,
        appointment_at: NaiveDateTime,
    ) -> Result<Option<Appointment>, RepositoryError>;
    async fn update(&self, appointment: &Appointment) -> Result<Appointment, RepositoryError>;
    async fn find_by_patient_id(
        &self,
        patient_id: i32,
    ) -> Result<Vec<Appointment>, RepositoryError>;
}
