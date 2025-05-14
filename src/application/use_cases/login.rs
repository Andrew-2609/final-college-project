use bcrypt::verify;

use crate::{
    application::{
        errors::admin_application_error::AdminApplicationError, security::jwt::jwt::create_jwt,
    },
    domain::repositories::admin_repository::AdminRepository,
    presentation::dtos::admin_dto::LoginDTO,
};

pub struct LoginUseCase<T: AdminRepository> {
    admin_repo: T,
}

impl<T: AdminRepository> LoginUseCase<T> {
    pub fn new(admin_repo: T) -> Self {
        Self { admin_repo }
    }

    pub async fn execute(&self, input: LoginDTO) -> Result<String, AdminApplicationError> {
        let admin = self.admin_repo.find_by_email(input.email.clone()).await?;

        if admin.is_none() {
            return Err(AdminApplicationError::NotFound(input.email));
        }

        let admin = admin.unwrap();
        let admin_email = admin.email.clone();
        let password_hash = admin.password_hash.clone();

        if verify(&input.password, &password_hash).unwrap_or(false) {
            let token = create_jwt(admin_email.clone());
            if token.is_none() {
                return Err(AdminApplicationError::LoginFailed(format!(
                    "Could not generate JWT token for admin {}",
                    admin_email
                )));
            }
            return Ok(token.unwrap());
        };

        Err(AdminApplicationError::LoginFailed(format!(
            "The provided credentials are invalid: {admin_email}"
        )))
    }
}
