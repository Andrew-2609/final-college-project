use crate::domain::{entities::patient::Patient, errors::repository_error::RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait PatientRepository {
    async fn save(&self, patient: &Patient) -> Result<i32, RepositoryError>;
}
