use std::fmt;

use crate::domain::errors::repository_error::RepositoryError;

#[derive(Debug)]
pub enum AdminApplicationError {
    Unexpected(String),
    NotFound(String),
    LoginFailed(String),
}

impl fmt::Display for AdminApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdminApplicationError::Unexpected(msg) => {
                write!(f, "An unexpected error occurred: {msg}")
            }
            AdminApplicationError::NotFound(email) => {
                write!(
                    f,
                    "An admin with the following email was not found: {email}"
                )
            }
            AdminApplicationError::LoginFailed(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

impl std::error::Error for AdminApplicationError {}

impl From<RepositoryError> for AdminApplicationError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::DatabaseError(msg) => AdminApplicationError::Unexpected(msg),
        }
    }
}
