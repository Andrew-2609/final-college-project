use crate::schema;
use crate::schema::appointments::dsl::{appointment_at, appointments, canceled, id, patient_id};
use crate::{
    domain::{
        entities::appointment::Appointment, errors::repository_error::RepositoryError,
        repositories::appointment_repository::AppointmentRepository,
    },
    infrastructure::db::connection::{DBPool, establish_connection},
};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::{dsl::exists, prelude::*, select};
use std::sync::Arc;

#[derive(Clone)]
pub struct PostgresAppointmentRepository {
    pool: DBPool,
}

impl PostgresAppointmentRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        Self {
            pool: establish_connection(&database_url),
        }
    }
}

#[async_trait]
impl AppointmentRepository for Arc<PostgresAppointmentRepository> {
    async fn exists_by_patient_id_and_appointment_at(
        &self,
        input_patient_id: i32,
        input_appointment_at: NaiveDateTime,
    ) -> Result<bool, RepositoryError> {
        let exists_by_patient_id_and_appointment_at = select(exists(
            appointments
                .filter(patient_id.eq(input_patient_id))
                .filter(appointment_at.eq(input_appointment_at))
                .filter(canceled.eq(false)),
        ))
        .get_result(&mut self.pool.get().unwrap())?;

        Ok(exists_by_patient_id_and_appointment_at)
    }

    async fn save(&self, appointment: &Appointment) -> Result<Appointment, RepositoryError> {
        let inserted_appointment = diesel::insert_into(schema::appointments::table)
            .values(appointment.clone())
            .get_result::<Appointment>(&mut self.pool.get().unwrap())?;

        Ok(inserted_appointment)
    }

    async fn find_by_patient_id_and_appointment_at(
        &self,
        input_patient_id: i32,
        input_appointment_at: NaiveDateTime,
    ) -> Result<Option<Appointment>, RepositoryError> {
        let appointment = appointments
            .filter(patient_id.eq(input_patient_id))
            .filter(appointment_at.eq(input_appointment_at))
            .filter(canceled.eq(false))
            .first::<Appointment>(&mut self.pool.get().unwrap())
            .optional()?;

        Ok(appointment)
    }

    async fn update(&self, appointment: &Appointment) -> Result<Appointment, RepositoryError> {
        let appointment_id: Option<i32> = appointment.id.clone().into();
        let appointment_id = appointment_id.unwrap_or(0);

        let updated_appointment = diesel::update(schema::appointments::table)
            .filter(id.eq(appointment_id))
            .set(appointment.clone())
            .get_result::<Appointment>(&mut self.pool.get().unwrap())?;

        Ok(updated_appointment)
    }

    async fn find_by_patient_id(
        &self,
        input_patient_id: i32,
    ) -> Result<Vec<Appointment>, RepositoryError> {
        let found_appointments = appointments
            .filter(patient_id.eq(input_patient_id))
            .load::<Appointment>(&mut self.pool.get().unwrap())?;

        Ok(found_appointments)
    }
}
