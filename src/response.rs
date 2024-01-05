use chrono::prelude::*;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredGuest {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub verified: bool,
    pub phone_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct GuestData {
    pub guest: FilteredGuest,
}

#[derive(Serialize, Debug)]
pub struct GuestResponse {
    pub status: String,
    pub data: GuestData,
}
