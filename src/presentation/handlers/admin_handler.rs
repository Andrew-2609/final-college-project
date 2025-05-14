use actix_web::{HttpResponse, post, web};

use crate::{
    application::use_cases::login::LoginUseCase, infrastructure::web::AppState,
    presentation::dtos::admin_dto::LoginDTO,
};

#[post("/login")]
pub async fn login_handler(
    app_state: web::Data<AppState>,
    input: web::Json<LoginDTO>,
) -> HttpResponse {
    match LoginUseCase::new(app_state.admin_repo.clone())
        .execute(input.into_inner())
        .await
    {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::Unauthorized().json(err.to_string()),
    }
}
