use crate::{
    domain::{errors::appointment_entity_error::AppointmentEntityError, value_objects::id::ID},
    schema::appointments,
};
use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};

#[derive(Clone, Insertable, Queryable)]
#[diesel(table_name = appointments)]
pub struct Appointment {
    #[diesel(serialize_as = Option<i32>, deserialize_as = i32)]
    pub id: ID,
    pub patient_id: i32,
    pub appointment_at: NaiveDateTime,
    pub specialty: String,
    pub notes: Option<String>,
}

impl Appointment {
    pub fn new(
        patient_id: i32,
        appointment_at: NaiveDateTime,
        specialty: String,
        notes: Option<String>,
    ) -> Result<Self, AppointmentEntityError> {
        if patient_id <= 0 {
            return Err(AppointmentEntityError::InvalidPatientId(patient_id));
        }

        Ok(Self {
            id: ID::New,
            patient_id,
            appointment_at,
            specialty,
            notes,
        })
    }
}
