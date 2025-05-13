use actix_web::{App, HttpServer, middleware::Logger, web};
use log::info;

use crate::{
    infrastructure::repositories::postgres_patient_repository::PostgresPatientRepository,
    presentation::routes,
};

pub async fn run() -> std::io::Result<()> {
    let patient_repo = PostgresPatientRepository::new();
    let app_data = web::Data::new(patient_repo);

    info!("Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes::patient_routes::routes)
    })
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}
