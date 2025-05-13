use actix_web::{HttpResponse, ResponseError, post, web};

use crate::{
    application::use_cases::register_patient::RegisterPatientUseCase,
    infrastructure::repositories::postgres_patient_repository::PostgresPatientRepository,
    presentation::{
        dtos::patient_dto::CreatePatientDTO, errors::patient_http_error::PatientHttpError,
    },
};

#[post("")]
pub async fn register_patient_handler(
    repo: web::Data<PostgresPatientRepository>,
    input: web::Json<CreatePatientDTO>,
) -> HttpResponse {
    match RegisterPatientUseCase::new(repo.into_inner())
        .execute(input.into_inner())
        .await
    {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => PatientHttpError::from(err).error_response(),
    }
}
