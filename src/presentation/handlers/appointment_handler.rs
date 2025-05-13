use actix_web::{HttpResponse, ResponseError, post, web};

use crate::{
    application::use_cases::register_appointment::RegisterAppointmentUseCase,
    infrastructure::web::AppState,
    presentation::{
        dtos::appointment_dto::{CreateAppointmentDTO, LoadedAppointmentDTO},
        errors::appointment_http_error::AppointmentHttpError,
    },
};

#[post("")]
pub async fn register_appointment_handler(
    app_state: web::Data<AppState>,
    input: web::Json<CreateAppointmentDTO>,
) -> HttpResponse {
    match RegisterAppointmentUseCase::new(
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
