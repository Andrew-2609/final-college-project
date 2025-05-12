use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::DatabaseError(msg) => {
                write!(f, "A database error occurred: {msg}")
            }
        }
    }
}

impl std::error::Error for RepositoryError {}

#[cfg(test)]
mod test {
    use super::RepositoryError;

    #[test]
    fn display() {
        let error_msg = "Connection lost";
        let err = RepositoryError::DatabaseError(error_msg.to_string());
        let err = err.to_string();

        assert_eq!(err, "A database error occurred: ".to_owned() + error_msg);
    }
}
