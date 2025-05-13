use std::fmt;

#[derive(Debug)]
pub enum AppointmentEntityError {
    InvalidPatientId(i32),
    InvalidAppointmentAt(String),
}

impl fmt::Display for AppointmentEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppointmentEntityError::InvalidPatientId(patient_id) => {
                write!(f, "Invalid patient ID: {patient_id}")
            }
            AppointmentEntityError::InvalidAppointmentAt(msg) => {
                write!(f, "Invalid appointment at: {msg}")
            }
        }
    }
}

impl std::error::Error for AppointmentEntityError {}

impl From<chrono::ParseError> for AppointmentEntityError {
    fn from(value: chrono::ParseError) -> Self {
        AppointmentEntityError::InvalidAppointmentAt(value.to_string())
    }
}
