use crate::domain::errors::repository_error::RepositoryError;

impl From<diesel::result::Error> for RepositoryError {
    fn from(value: diesel::result::Error) -> Self {
        RepositoryError::DatabaseError(value.to_string())
    }
}
