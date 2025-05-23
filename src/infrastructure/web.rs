use std::sync::Arc;

use actix_web::{App, HttpServer, middleware::Logger, web};
use log::info;

use crate::{
    infrastructure::repositories::{
        postgres_appointment_repository::PostgresAppointmentRepository,
        postgres_patient_repository::PostgresPatientRepository,
    },
    presentation::routes,
};

use super::repositories::postgres_admin_repository::PostgresAdminRepository;

pub struct AppState {
    pub patient_repo: Arc<PostgresPatientRepository>,
    pub appointment_repo: Arc<PostgresAppointmentRepository>,
    pub admin_repo: Arc<PostgresAdminRepository>,
}

pub async fn run() -> std::io::Result<()> {
    let patient_repo = Arc::new(PostgresPatientRepository::new());
    let appointment_repo = Arc::new(PostgresAppointmentRepository::new());
    let admin_repo = Arc::new(PostgresAdminRepository::new());

    let app_data = web::Data::new(AppState {
        patient_repo,
        appointment_repo,
        admin_repo,
    });

    info!("Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes::patient_routes::patient_routes)
            .configure(routes::appointment_routes::appointment_routes)
            .configure(routes::admin_routes::admin_routes)
    })
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}
