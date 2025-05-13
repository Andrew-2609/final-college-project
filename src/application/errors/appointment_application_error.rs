use std::fmt;

use crate::domain::errors::{
    appointment_entity_error::AppointmentEntityError, repository_error::RepositoryError,
};

#[derive(Debug)]
pub enum AppointmentApplicationError {
    Constraint(String),
    Unexpected(String),
    PatientNotFound(String),
}

impl fmt::Display for AppointmentApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppointmentApplicationError::Constraint(msg) => {
                write!(f, "{}", msg)
            }
            AppointmentApplicationError::Unexpected(msg) => {
                write!(f, "An unexpected error occurred: {msg}")
            }
            AppointmentApplicationError::PatientNotFound(cpf) => {
                write!(f, "A patient with the following CPF was not found: {cpf}")
            }
        }
    }
}

impl std::error::Error for AppointmentApplicationError {}

impl From<RepositoryError> for AppointmentApplicationError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::DatabaseError(msg) => AppointmentApplicationError::Unexpected(msg),
        }
    }
}

impl From<AppointmentEntityError> for AppointmentApplicationError {
    fn from(value: AppointmentEntityError) -> Self {
        match value {
            err => AppointmentApplicationError::Constraint(err.to_string()),
        }
    }
}

impl From<chrono::ParseError> for AppointmentApplicationError {
    fn from(value: chrono::ParseError) -> Self {
        AppointmentApplicationError::Unexpected(value.to_string())
    }
}
