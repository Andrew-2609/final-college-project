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
    pub canceled: bool,
    pub canceled_at: Option<String>,
    pub cancellation_reason: Option<String>,
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
                canceled: value.canceled,
                canceled_at: value.canceled_at.map(|dt| dt.to_string()),
                cancellation_reason: value.cancellation_reason,
            }),
            ID::New => None,
        }
    }
}

#[derive(Serialize)]
pub struct LoadedAppointmentsDTO(Vec<LoadedAppointmentDTO>);

impl LoadedAppointmentsDTO {
    pub fn push(&mut self, appointment_dto: LoadedAppointmentDTO) {
        self.0.push(appointment_dto);
    }

    pub fn collect(self) -> Vec<LoadedAppointmentDTO> {
        self.0
    }
}

impl From<Vec<Appointment>> for LoadedAppointmentsDTO {
    fn from(value: Vec<Appointment>) -> Self {
        let mut result: LoadedAppointmentsDTO = LoadedAppointmentsDTO(Vec::new());

        for appointment in value {
            if let Some(appointment_dto) = Option::<LoadedAppointmentDTO>::from(appointment) {
                result.push(appointment_dto);
            }
        }

        result
    }
}

#[derive(Deserialize)]
pub struct CancelAppointmentDTO {
    pub patient_cpf: String,
    pub appointment_at: String,
    pub cancellation_reason: Option<String>,
}
