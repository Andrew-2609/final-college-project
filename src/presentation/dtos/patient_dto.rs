use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct CreatePatientDTO {
    pub name: String,
    pub cpf: String,
}
