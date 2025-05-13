use crate::domain::{entities::patient::Patient, errors::repository_error::RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait PatientRepository {
    async fn exists_by_cpf(&self, cpf: &str) -> Result<bool, RepositoryError>;
    async fn save(&self, patient: &Patient) -> Result<i32, RepositoryError>;
}
