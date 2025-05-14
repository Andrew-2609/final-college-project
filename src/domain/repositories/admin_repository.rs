use async_trait::async_trait;

use crate::domain::{entities::admin::Admin, errors::repository_error::RepositoryError};

#[async_trait]
pub trait AdminRepository {
    async fn find_by_email(&self, email: String) -> Result<Option<Admin>, RepositoryError>;
}
