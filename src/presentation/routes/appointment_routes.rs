use actix_web::web;

use crate::presentation::handlers::appointment_handler::book_appointment_handler;

pub fn appointment_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1/appointments").service(book_appointment_handler));
}
