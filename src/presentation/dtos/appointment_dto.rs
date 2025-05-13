use serde::{Deserialize, Serialize};

use crate::domain::{entities::appointment::Appointment, value_objects::id::ID};

#[derive(Deserialize)]
pub struct BookAppointmentDTO {
    pub patient_cpf: String,
    pub appointment_at: String,
    pub specialty: String,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct LoadedAppointmentDTO {
    pub id: i32,
    pub patient_id: i32,
    pub appointment_at: String,
    pub specialty: String,
    pub notes: Option<String>,
}

impl From<Appointment> for Option<LoadedAppointmentDTO> {
    fn from(value: Appointment) -> Self {
        match value.id {
            ID::Existing(id) => Self::Some(LoadedAppointmentDTO {
                id,
                patient_id: value.patient_id,
                appointment_at: value.appointment_at.to_string(),
                specialty: value.specialty,
                notes: value.notes,
            }),
            ID::New => None,
        }
    }
}
