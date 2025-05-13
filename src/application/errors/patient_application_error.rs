use std::fmt;

use crate::domain::errors::repository_error::RepositoryError;

#[derive(Debug, PartialEq)]
pub enum PatientApplicationError {
    Conflict(String),
    Unexpected(String),
}

impl fmt::Display for PatientApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatientApplicationError::Conflict(msg) => {
                write!(
                    f,
                    "The following conflict occurred when writing a patient: {msg}"
                )
            }
            PatientApplicationError::Unexpected(msg) => {
                write!(f, "An unexpected error occurred: {msg}")
            }
        }
    }
}

impl std::error::Error for PatientApplicationError {}

impl From<RepositoryError> for PatientApplicationError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::DatabaseError(msg) => PatientApplicationError::Unexpected(msg),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::errors::patient_application_error::PatientApplicationError,
        domain::errors::repository_error::RepositoryError,
    };

    #[test]
    fn patient_application_error_conflict_display() {
        let err_msg = "cpf already taken";
        let err = PatientApplicationError::Conflict(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            "The following conflict occurred when writing a patient: ".to_owned() + err_msg
        );
    }

    #[test]
    fn user_application_error_unexpected_display() {
        let err_msg = "database error";
        let err = PatientApplicationError::Unexpected(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(err, "An unexpected error occurred: ".to_owned() + err_msg);
    }

    #[test]
    fn patient_application_error_from_repository_error() {
        let err_msg = "database error";
        let repo_err = RepositoryError::DatabaseError(err_msg.to_string());
        let err: PatientApplicationError = repo_err.into();

        assert_eq!(
            err,
            PatientApplicationError::Unexpected(err_msg.to_string())
        );
    }
}
