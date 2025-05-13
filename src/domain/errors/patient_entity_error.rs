use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PatientEntityError {
    InvalidId(i32),
}

impl fmt::Display for PatientEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatientEntityError::InvalidId(patient_id) => {
                write!(f, "An invalid ID was given for a patient: {patient_id}")
            }
        }
    }
}

impl std::error::Error for PatientEntityError {}

#[cfg(test)]
mod test {
    use super::PatientEntityError;

    #[test]
    fn display() {
        let patient_id: i32 = 0;
        let err = PatientEntityError::InvalidId(patient_id);
        let err = err.to_string();

        assert_eq!(
            err,
            format!("An invalid ID was given for a patient: {patient_id}")
        );
    }
}
