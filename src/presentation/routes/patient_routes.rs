use actix_web::web;

use crate::presentation::handlers::patient_handler::register_patient_handler;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1/patients").service(register_patient_handler));
}
