use diesel::prelude::Insertable;

use crate::{
    domain::{errors::patient_entity_error::PatientEntityError, value_objects::id::ID},
    presentation::dtos::patient_dto::CreatePatientDTO,
    schema::patients,
};

#[derive(Clone, Debug, Insertable, PartialEq)]
#[diesel(table_name = patients)]
pub struct Patient {
    #[diesel(serialize_as = Option<i32>, deserialize_as = i32)]
    pub id: ID,
    pub name: String,
    pub cpf: String,
}

impl Patient {
    pub fn new(name: String, cpf: String) -> Self {
        Self {
            id: ID::New,
            name,
            cpf,
        }
    }

    pub fn restore(id: i32, name: String, cpf: String) -> Result<Self, PatientEntityError> {
        if id <= 0 {
            return Err(PatientEntityError::InvalidId(id));
        }

        Ok(Self {
            id: ID::Existing(id),
            name,
            cpf,
        })
    }
}

impl From<CreatePatientDTO> for Patient {
    fn from(value: CreatePatientDTO) -> Self {
        Self::new(value.name, value.cpf)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{
            entities::patient::Patient, errors::patient_entity_error::PatientEntityError,
            value_objects::id::ID,
        },
        presentation::dtos::patient_dto::CreatePatientDTO,
    };

    #[test]
    fn new() {
        let name = "Andrew";
        let cpf = "00011122233";

        let patient = Patient::new(name.to_string(), cpf.to_string());

        assert_eq!(patient.id, ID::New);
        assert_eq!(patient.name, name);
        assert_eq!(patient.cpf, cpf);
    }

    #[test]
    fn restore_non_positive_id_error() {
        let id = 0;

        let patient = Patient::restore(id, "Andrew".to_string(), "00011122233".to_string());

        assert_eq!(patient, Err(PatientEntityError::InvalidId(0)))
    }

    #[test]
    fn restore_ok() {
        let id = 42;
        let name = "Andrew";
        let cpf = "00011122233";

        let patient = Patient::restore(id, name.to_string(), cpf.to_string()).unwrap();

        assert_eq!(patient.id, ID::Existing(42));
        assert_eq!(patient.name, name);
        assert_eq!(patient.cpf, cpf);
    }

    #[test]
    fn from_create_patient_dto() {
        let dto = CreatePatientDTO {
            name: String::from("Andrew"),
            cpf: String::from("00011122233"),
        };

        let patient: Patient = dto.clone().into();

        assert_eq!(patient.id, ID::New);
        assert_eq!(patient.name, dto.name);
        assert_eq!(patient.cpf, dto.cpf);
    }
}
