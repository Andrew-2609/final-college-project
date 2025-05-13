use actix_web::web;

use crate::presentation::handlers::appointment_handler::register_appointment_handler;

pub fn appointment_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1/appointments").service(register_appointment_handler));
}
