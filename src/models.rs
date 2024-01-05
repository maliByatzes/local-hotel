use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Guest {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub password: String,
    pub verified: bool,
    pub phone_number: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterGuestSchema {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginGuestSchema {
    pub email_address: String,
    pub password: String,
}
