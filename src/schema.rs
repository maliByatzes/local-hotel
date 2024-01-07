use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

// TODO: Validation
#[derive(Debug, Deserialize)]
pub struct RegisterGuestSchema {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub password: String,
    pub phone_number: String,
}

// TODO: Validation
#[derive(Debug, Deserialize)]
pub struct LoginGuestSchema {
    pub email_address: String,
    pub password: String,
}
