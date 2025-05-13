use actix_web::web;

use crate::presentation::handlers::patient_handler::{
    delete_patient_by_cpf_handler, find_patient_by_cpf_handler, register_patient_handler,
    update_patient_by_cpf_handler,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/patients")
            .service(register_patient_handler)
            .service(find_patient_by_cpf_handler)
            .service(update_patient_by_cpf_handler)
            .service(delete_patient_by_cpf_handler),
    );
}
