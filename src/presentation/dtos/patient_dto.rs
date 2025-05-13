use serde::{Deserialize, Serialize};

use crate::domain::{entities::patient::Patient, value_objects::id::ID};

#[derive(Clone, Deserialize)]
pub struct CreatePatientDTO {
    pub name: String,
    pub cpf: String,
}

#[derive(Serialize)]
pub struct LoadedPatientDTO {
    pub id: i32,
    pub name: String,
    pub cpf: String,
}

impl From<Patient> for Option<LoadedPatientDTO> {
    fn from(value: Patient) -> Self {
        match value.id {
            ID::Existing(id) => Self::Some(LoadedPatientDTO {
                id,
                name: value.name,
                cpf: value.cpf,
            }),
            ID::New => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{entities::patient::Patient, value_objects::id::ID},
        presentation::dtos::patient_dto::LoadedPatientDTO,
    };

    #[test]
    fn from_patient_into_optional_loaded_patient_dto() {
        let id = 42;
        let name = "Andrew";
        let cpf = "00011122233";

        let new_patient = Patient::new(name.to_string(), cpf.to_string());

        let loaded_patient_dto: Option<LoadedPatientDTO> = new_patient.clone().into();

        assert!(loaded_patient_dto.is_none());

        let mut existing_patient = new_patient;
        existing_patient.id = ID::Existing(id);

        let loaded_patient_dto: Option<LoadedPatientDTO> = existing_patient.into();
        let loaded_patient_dto = loaded_patient_dto.unwrap();

        assert_eq!(loaded_patient_dto.id, id);
        assert_eq!(loaded_patient_dto.name, name);
        assert_eq!(loaded_patient_dto.cpf, cpf);
    }
}
