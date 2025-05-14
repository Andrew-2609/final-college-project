use actix_web::web;

use crate::presentation::handlers::admin_handler::login_handler;

pub fn admin_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1").service(login_handler));
}
