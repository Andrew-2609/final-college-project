use crate::domain::{entities::patient::Patient, errors::repository_error::RepositoryError};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait PatientRepository {
    async fn exists_by_cpf(&self, cpf: &str) -> Result<bool, RepositoryError>;
    async fn save(&self, patient: &Patient) -> Result<i32, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Patient>, RepositoryError>;
    async fn find_by_cpf(&self, cpf: String) -> Result<Option<Patient>, RepositoryError>;
    async fn update(&self, patient: &Patient) -> Result<Patient, RepositoryError>;
    async fn delete_by_cpf(&self, cpf: String) -> Result<(), RepositoryError>;
}
