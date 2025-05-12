use crate::domain::{errors::patient_entity_error::PatientEntityError, value_objects::id::ID};

pub struct Patient {
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

#[cfg(test)]
mod test {
    use crate::domain::{entities::patient::Patient, value_objects::id::ID};

    #[test]
    fn new() {
        let name = "Andrew";
        let cpf = "00011122233";

        let patient = Patient::new(name.to_string(), cpf.to_string());

        assert_eq!(patient.id, ID::New);
        assert_eq!(patient.name, name);
        assert_eq!(patient.cpf, cpf);
    }
}
