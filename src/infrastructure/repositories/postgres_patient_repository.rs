use crate::{
    domain::{
        entities::patient::Patient, errors::repository_error::RepositoryError,
        repositories::patient_repository::PatientRepository,
    },
    infrastructure::db::connection::{DBPool, establish_connection},
    schema::{
        self,
        patients::dsl::{cpf, id, patients},
    },
};
use async_trait::async_trait;
use diesel::{dsl::exists, prelude::*, select};
use std::sync::Arc;

pub struct PostgresPatientRepository {
    pool: DBPool,
}

impl PostgresPatientRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        Self {
            pool: establish_connection(&database_url),
        }
    }
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(value: diesel::result::Error) -> Self {
        RepositoryError::DatabaseError(value.to_string())
    }
}

#[async_trait]
impl PatientRepository for Arc<PostgresPatientRepository> {
    async fn exists_by_cpf(&self, input_cpf: &str) -> Result<bool, RepositoryError> {
        let exists_by_cpf = select(exists(patients.filter(cpf.eq(input_cpf))))
            .get_result(&mut self.pool.get().unwrap())?;

        Ok(exists_by_cpf)
    }

    async fn save(&self, patient: &Patient) -> Result<i32, RepositoryError> {
        let inserted_patient_id = diesel::insert_into(schema::patients::table)
            .values(patient.clone())
            .returning(id)
            .get_result(&mut self.pool.get().unwrap())?;

        Ok(inserted_patient_id)
    }

    async fn find_by_cpf(&self, input_cpf: String) -> Result<Option<Patient>, RepositoryError> {
        let patient = patients
            .filter(cpf.eq(input_cpf))
            .first::<Patient>(&mut self.pool.get().unwrap())
            .optional()?;

        Ok(patient)
    }
}
