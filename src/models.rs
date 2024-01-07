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

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Booking {
    pub id: i32,
    pub guest_id: i32,
    pub payment_status_id: i32,
    pub checkin_date: DateTime<Utc>,
    pub checkout_date: DateTime<Utc>,
    pub num_adults: i32,
    pub num_children: i32,
    pub booking_amount: f64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct PaymentStatus {
    pub id: i32,
    pub payment_status_name: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
