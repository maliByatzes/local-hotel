use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookingSchema {
    pub checkin_date: NaiveDate,
    pub checkout_date: NaiveDate,
    pub num_adults: i32,
    pub num_children: i32,
    pub booking_amount: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBookingSchema {
    pub checkin_date: Option<NaiveDate>,
    pub checkout_date: Option<NaiveDate>,
    pub num_adults: Option<i32>,
    pub num_children: Option<i32>,
    pub booking_amount: Option<BigDecimal>,
}
