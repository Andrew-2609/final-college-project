use std::fmt;

use actix_web::{HttpResponse, ResponseError, body::BoxBody};

use crate::application::errors::patient_application_error::PatientApplicationError;

#[derive(Debug, PartialEq)]
pub enum PatientHttpError {
    Constraint(String),
    Internal(String),
    NotFound(String),
}

impl fmt::Display for PatientHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatientHttpError::Constraint(msg) => {
                write!(f, "A constraint error occurred for the patient: {msg}")
            }
            PatientHttpError::Internal(msg) => {
                write!(f, "An internal error occurred for the patient: {msg}")
            }
            PatientHttpError::NotFound(msg) => {
                write!(f, "The patient could not be found: {msg}")
            }
        }
    }
}

impl std::error::Error for PatientHttpError {}

impl From<PatientApplicationError> for PatientHttpError {
    fn from(value: PatientApplicationError) -> Self {
        match value {
            PatientApplicationError::Conflict(msg) => Self::Constraint(msg),
            PatientApplicationError::Unexpected(msg) => Self::Internal(msg),
            PatientApplicationError::NotFound(msg) => Self::NotFound(msg),
        }
    }
}

impl ResponseError for PatientHttpError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            PatientHttpError::Constraint(_) => {
                HttpResponse::UnprocessableEntity().json(self.to_string())
            }
            PatientHttpError::Internal(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            PatientHttpError::NotFound(_) => HttpResponse::NotFound().json(self.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::{ResponseError, body::MessageBody, http::StatusCode};

    use crate::application::errors::patient_application_error::PatientApplicationError;

    use super::PatientHttpError;

    #[test]
    fn display_constraint_error() {
        let err_msg = "Constraint X violated";
        let err = PatientHttpError::Constraint(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            format!("A constraint error occurred for the patient: {err_msg}")
        );
    }

    #[test]
    fn display_internal_error() {
        let err_msg = "Database error";
        let err = PatientHttpError::Internal(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            format!("An internal error occurred for the patient: {err_msg}")
        );
    }

    #[test]
    fn from_patient_application_conflict_error() {
        let err_msg = "Constraint X violated";
        let application_err = PatientApplicationError::Conflict(err_msg.to_string());
        let err: PatientHttpError = application_err.into();

        assert_eq!(err, PatientHttpError::Constraint(err_msg.to_string()));
    }

    #[test]
    fn from_patient_application_internal_error() {
        let err_msg = "Database error";
        let application_err = PatientApplicationError::Unexpected(err_msg.to_string());
        let err: PatientHttpError = application_err.into();

        assert_eq!(err, PatientHttpError::Internal(err_msg.to_string()));
    }

    #[test]
    fn from_patient_application_not_found_error() {
        let err_msg = "Patient not found";
        let application_err = PatientApplicationError::NotFound(err_msg.to_string());
        let err: PatientHttpError = application_err.into();

        assert_eq!(err, PatientHttpError::NotFound(err_msg.to_string()));
    }

    #[test]
    fn constraint_error_response() -> Result<(), Box<dyn std::error::Error>> {
        let err = PatientHttpError::Constraint("Constraint X violated".to_string());

        let result = err.error_response();

        let result_status = result.status();
        let result_body = result.into_body().try_into_bytes().unwrap();
        let result_body = std::str::from_utf8(&result_body)?;

        assert_eq!(result_status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(result_body.replace("\"", ""), err.to_string());

        Ok(())
    }

    #[test]
    fn internal_error_response() -> Result<(), Box<dyn std::error::Error>> {
        let err = PatientHttpError::Internal("Database error".to_string());

        let result = err.error_response();

        let result_status = result.status();
        let result_body = result.into_body().try_into_bytes().unwrap();
        let result_body = std::str::from_utf8(&result_body)?;

        assert_eq!(result_status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result_body.replace("\"", ""), err.to_string());

        Ok(())
    }

    #[test]
    fn not_found_error_response() -> Result<(), Box<dyn std::error::Error>> {
        let err = PatientHttpError::NotFound("Patient not found".to_string());

        let result = err.error_response();

        let result_status = result.status();
        let result_body = result.into_body().try_into_bytes().unwrap();
        let result_body = std::str::from_utf8(&result_body)?;

        assert_eq!(result_status, StatusCode::NOT_FOUND);
        assert_eq!(result_body.replace("\"", ""), err.to_string());

        Ok(())
    }
}
