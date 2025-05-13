use std::fmt;

use crate::domain::errors::{
    appointment_entity_error::AppointmentEntityError, repository_error::RepositoryError,
};

#[derive(Debug)]
pub enum AppointmentApplicationError {
    Constraint(String),
    Unexpected(String),
    PatientNotFound(i32),
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
            AppointmentApplicationError::PatientNotFound(id) => {
                write!(f, "A patient with the following ID was not found: {id}")
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
