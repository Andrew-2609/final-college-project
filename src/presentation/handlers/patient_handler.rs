use actix_web::{
    HttpResponse, ResponseError, delete, get, post, put,
    web::{self, Path},
};

use crate::{
    application::use_cases::{
        delete_patient_by_cpf::DeletePatientByCpfUseCase,
        find_patient_by_cpf::FindPatientByCpfUseCase,
        list_appointments_by_patient_cpf::ListAppointmentsByPatientCpfUseCase,
        register_patient::RegisterPatientUseCase, update_patient_by_cpf::UpdatePatientByCpfUseCase,
    },
    infrastructure::web::AppState,
    presentation::{
        dtos::{
            appointment_dto::LoadedAppointmentsDTO,
            patient_dto::{CreatePatientDTO, LoadedPatientDTO, UpdatePatientDTO},
        },
        errors::{
            appointment_http_error::AppointmentHttpError, patient_http_error::PatientHttpError,
        },
    },
};

#[post("")]
pub async fn register_patient_handler(
    app_state: web::Data<AppState>,
    input: web::Json<CreatePatientDTO>,
) -> HttpResponse {
    match RegisterPatientUseCase::new(app_state.patient_repo.clone())
        .execute(input.into_inner())
        .await
    {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => PatientHttpError::from(err).error_response(),
    }
}

#[get("/{cpf}")]
pub async fn find_patient_by_cpf_handler(
    app_state: web::Data<AppState>,
    path: Path<String>,
) -> HttpResponse {
    let cpf = path.into_inner();

    let result = FindPatientByCpfUseCase::new(app_state.patient_repo.clone())
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

#[put("/{cpf}")]
pub async fn update_patient_by_cpf_handler(
    app_state: web::Data<AppState>,
    path: Path<String>,
    input: web::Json<UpdatePatientDTO>,
) -> HttpResponse {
    match UpdatePatientByCpfUseCase::new(app_state.patient_repo.clone())
        .execute(path.into_inner(), input.into_inner())
        .await
    {
        Ok(patient) => {
            let loaded_patient: Option<LoadedPatientDTO> = patient.into();
            HttpResponse::Ok().json(loaded_patient)
        }
        Err(err) => PatientHttpError::from(err).error_response(),
    }
}

#[delete("/{cpf}")]
pub async fn delete_patient_by_cpf_handler(
    app_state: web::Data<AppState>,
    path: Path<String>,
) -> HttpResponse {
    match DeletePatientByCpfUseCase::new(app_state.patient_repo.clone())
        .execute(path.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(err) => PatientHttpError::from(err).error_response(),
    }
}

#[get("/{cpf}/appointments")]
pub async fn list_appointments_by_patient_cpf_handler(
    app_state: web::Data<AppState>,
    path: Path<String>,
) -> HttpResponse {
    match ListAppointmentsByPatientCpfUseCase::new(
        app_state.patient_repo.clone(),
        app_state.appointment_repo.clone(),
    )
    .execute(path.into_inner())
    .await
    {
        Ok(appointments) => {
            let loaded_appointments: LoadedAppointmentsDTO = appointments.into();
            HttpResponse::Ok().json(loaded_appointments)
        }
        Err(err) => AppointmentHttpError::from(err).error_response(),
    }
}
