use crate::{
    domain::{errors::appointment_entity_error::AppointmentEntityError, value_objects::id::ID},
    schema::appointments,
};
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable};

#[derive(AsChangeset, Clone, Insertable, Queryable)]
#[diesel(table_name = appointments)]
pub struct Appointment {
    #[diesel(serialize_as = Option<i32>, deserialize_as = i32)]
    pub id: ID,
    pub patient_id: i32,
    pub appointment_at: NaiveDateTime,
    pub specialty: String,
    pub notes: Option<String>,
    pub canceled: bool,
    pub canceled_at: Option<NaiveDateTime>,
    pub cancellation_reason: Option<String>,
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
            canceled: false,
            canceled_at: None,
            cancellation_reason: None,
        })
    }

    pub fn cancel(&mut self, cancellation_reason: String) {
        self.canceled = true;
        self.canceled_at = Some(chrono::Local::now().naive_utc());
        self.cancellation_reason = Some(cancellation_reason);
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }
}
