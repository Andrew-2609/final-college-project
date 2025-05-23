use std::fmt;

use actix_web::{HttpResponse, ResponseError, body::BoxBody};

use crate::application::errors::appointment_application_error::AppointmentApplicationError;

#[derive(Debug)]
pub enum AppointmentHttpError {
    Constraint(String),
    Internal(String),
    NotFound(String),
    PatientNotFound(String),
}

impl fmt::Display for AppointmentHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppointmentHttpError::Constraint(msg) => {
                write!(f, "A constraint error occurred for the appointment: {msg}")
            }
            AppointmentHttpError::Internal(msg) => {
                write!(f, "An internal error occurred for the appointment: {msg}")
            }
            AppointmentHttpError::NotFound(msg) => {
                write!(f, "The appointment could not be found: {msg}")
            }
            AppointmentHttpError::PatientNotFound(patient_cpf) => {
                write!(f, "The patient could not be found: {patient_cpf}")
            }
        }
    }
}

impl std::error::Error for AppointmentHttpError {}

impl From<AppointmentApplicationError> for AppointmentHttpError {
    fn from(value: AppointmentApplicationError) -> Self {
        match value {
            AppointmentApplicationError::PatientNotFound(patient_cpf) => {
                AppointmentHttpError::PatientNotFound(patient_cpf)
            }
            AppointmentApplicationError::Constraint(msg) => AppointmentHttpError::Constraint(msg),
            AppointmentApplicationError::Unexpected(msg) => AppointmentHttpError::Internal(msg),
            AppointmentApplicationError::NotFound(msg) => AppointmentHttpError::NotFound(msg),
        }
    }
}

impl ResponseError for AppointmentHttpError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            AppointmentHttpError::Constraint(_) => {
                HttpResponse::UnprocessableEntity().json(self.to_string())
            }
            AppointmentHttpError::Internal(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            AppointmentHttpError::NotFound(_) => HttpResponse::NotFound().json(self.to_string()),
            AppointmentHttpError::PatientNotFound(_) => {
                HttpResponse::NotFound().json(self.to_string())
            }
        }
    }
}
