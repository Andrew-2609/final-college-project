use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    errors::Result as JwtResult,
};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &[u8] = b"secret"; // Should be stored in environment variable in a real project

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(email: String) -> Option<String> {
    let expiration = chrono::Utc::now().checked_add_signed(chrono::Duration::hours(24))?;
    let expiration = expiration.timestamp();

    let claims = Claims {
        sub: email,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .map_err(|err| err.to_string())
    .ok()
}

pub fn validate_jwt(token: String) -> JwtResult<TokenData<Claims>> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
}
