use crate::{
    domain::{
        entities::admin::Admin, errors::repository_error::RepositoryError,
        repositories::admin_repository::AdminRepository,
    },
    infrastructure::db::connection::{DBPool, establish_connection},
    schema::admins::dsl::{admins, email},
};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

pub struct PostgresAdminRepository {
    pool: DBPool,
}

impl PostgresAdminRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        Self {
            pool: establish_connection(&database_url),
        }
    }
}

#[async_trait]
impl AdminRepository for Arc<PostgresAdminRepository> {
    async fn find_by_email(&self, input_email: String) -> Result<Option<Admin>, RepositoryError> {
        let admin = admins
            .filter(email.eq(input_email))
            .first::<Admin>(&mut self.pool.get().unwrap())
            .optional()?;

        Ok(admin)
    }
}
