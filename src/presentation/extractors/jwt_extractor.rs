use actix_web::FromRequest;
use futures::future::{Ready, ready};

use crate::application::security::jwt::jwt::validate_jwt;

pub struct AuthenticatedAdmin {
    pub email: String,
}

impl FromRequest for AuthenticatedAdmin {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    if let Ok(token_data) = validate_jwt(token.to_string()) {
                        return ready(Ok(AuthenticatedAdmin {
                            email: token_data.claims.sub,
                        }));
                    }
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
    }
}
