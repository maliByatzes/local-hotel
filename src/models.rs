use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Booking {
    booking_id: i32,
    guest_id: i32,
    payment_status_id: i32,
    checkin_date: NaiveDateTime,
    checkout_date: NaiveDateTime,
    num_adults: i32,
    num_children: i32,
    booking_amount: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBooking {
    guest_id: i32,
    chekin_date: NaiveDateTime,
    checkout_date: NaiveDateTime,
    num_adults: i32,
    num_children: i32,
    booking_amount: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentStatus {
    guest_id: i32,
    payment_status_name: String,
}
