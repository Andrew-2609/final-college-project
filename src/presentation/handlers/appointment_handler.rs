use actix_web::{HttpResponse, ResponseError, patch, post, web};

use crate::{
    application::use_cases::{
        book_appointment::BookAppointmentUseCase, cancel_appointment::CancelAppointmentUseCase,
    },
    infrastructure::web::AppState,
    presentation::{
        dtos::appointment_dto::{BookAppointmentDTO, CancelAppointmentDTO, LoadedAppointmentDTO},
        errors::appointment_http_error::AppointmentHttpError,
    },
};

#[post("")]
pub async fn book_appointment_handler(
    app_state: web::Data<AppState>,
    input: web::Json<BookAppointmentDTO>,
) -> HttpResponse {
    match BookAppointmentUseCase::new(
        app_state.appointment_repo.clone(),
        app_state.patient_repo.clone(),
    )
    .execute(input.into_inner())
    .await
    {
        Ok(appointment) => {
            let loaded_appointment: Option<LoadedAppointmentDTO> = appointment.into();
            HttpResponse::Ok().json(loaded_appointment)
        }
        Err(err) => AppointmentHttpError::from(err).error_response(),
    }
}

#[patch("/cancellation")]
pub async fn cancel_appointment_handler(
    app_state: web::Data<AppState>,
    input: web::Json<CancelAppointmentDTO>,
) -> HttpResponse {
    match CancelAppointmentUseCase::new(
        app_state.appointment_repo.clone(),
        app_state.patient_repo.clone(),
    )
    .execute(input.into_inner())
    .await
    {
        Ok(appointment) => {
            let loaded_appointment: Option<LoadedAppointmentDTO> = appointment.into();
            HttpResponse::Ok().json(loaded_appointment)
        }
        Err(err) => AppointmentHttpError::from(err).error_response(),
    }
}
