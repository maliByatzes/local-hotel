use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::models::{Booking, CreateBooking};

// Configure health check handler
pub async fn health_check_handler() -> &'static str {
    "Local Hotel is alive and kicking"
}

// Configure create_booking handler
pub async fn create_booking(
    State(db): State<PgPool>,
    Json(payload): Json<CreateBooking>,
) -> impl IntoResponse {
    // Insert payload data to the database
    // Query the newly inserted booking
    // and return to client as json

    // sqlx::query!(
    //     r#"insert into booking (guest_id, payment_status_id, checkin_date, checkout_date, num_adults, num_children, booking_amount)
    //     values ($1, $2, $3, $4, $5, $6, $7)"#
    // )

    StatusCode::CREATED
}
