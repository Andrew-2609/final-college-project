use actix_web::{
    HttpResponse, ResponseError, get, post,
    web::{self, Path},
};

use crate::{
    application::use_cases::{
        find_patient_by_cpf::FindPatientByCpfUseCase, register_patient::RegisterPatientUseCase,
    },
    infrastructure::repositories::postgres_patient_repository::PostgresPatientRepository,
    presentation::{
        dtos::patient_dto::{CreatePatientDTO, LoadedPatientDTO},
        errors::patient_http_error::PatientHttpError,
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

#[get("/{cpf}")]
pub async fn find_patient_by_cpf_handler(
    repo: web::Data<PostgresPatientRepository>,
    path: Path<String>,
) -> HttpResponse {
    let cpf = path.into_inner();

    let result = FindPatientByCpfUseCase::new(repo.into_inner())
        .execute(cpf.clone())
        .await;

    match result {
        Ok(patient) => {
            if let Some(patient) = patient {
                let loaded_patient: Option<LoadedPatientDTO> = patient.into();
                HttpResponse::Ok().json(loaded_patient)
            } else {
                HttpResponse::NotFound().json(format!("Patient not found by CPF: {cpf}"))
            }
        }
        Err(err) => PatientHttpError::from(err).error_response(),
    }
}
