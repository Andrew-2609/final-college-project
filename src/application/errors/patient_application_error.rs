use std::fmt;

use crate::domain::errors::repository_error::RepositoryError;

#[derive(Debug, PartialEq)]
pub enum PatientApplicationError {
    CPFAlreadyTaken(String),
    Unexpected(String),
}

impl fmt::Display for PatientApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatientApplicationError::CPFAlreadyTaken(cpf) => {
                write!(f, "There's already a user registered with the CPF: {cpf}")
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
    fn patient_application_error_cpf_already_taken_display() {
        let cpf = "00011122233";
        let err = PatientApplicationError::CPFAlreadyTaken(cpf.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            format!("There's already a user registered with the CPF: {cpf}")
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
