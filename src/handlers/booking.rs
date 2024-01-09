use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use axum_macros::debug_handler;

use crate::{
    models::{Booking, Guest},
    schema::{CreateBookingSchema, FilterOptions, UpdateBookingSchema},
    AppState,
};

// Handler to get all the bookings of the guest
pub async fn booking_list_handler(
    Extension(guest): Extension<Guest>,
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Extract query options
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // Get the bookings from the database using the guest id
    let query_result = sqlx::query_as!(
        Booking,
        "select * from booking where guest_id = $1 order by id limit $2 offset $3",
        &guest.id,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    // Error checking from the query
    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Failed to fetch all bookings"
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let bookings = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": bookings.len(),
        "bookings": bookings
    });

    Ok(Json(json_response))
}

// Handler to get only one booking of the guest
pub async fn get_booking_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(guest): Extension<Guest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        Booking,
        "select * from booking where id = $1 and guest_id = $2",
        id,
        &guest.id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(booking) => {
            let booking_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "booking": booking
            })});

            return Ok(Json(booking_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Booking with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

// Handler to create a booking for the guest
#[debug_handler]
pub async fn create_booking_handler(
    Extension(guest): Extension<Guest>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateBookingSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Execute a SQL query to insert a new booking
    let query_result = sqlx::query_as!(
        Booking,
        "insert into booking 
            (
                guest_id, 
                payment_status_id, 
                checkin_date, 
                checkout_date, 
                num_adults, 
                num_children, 
                booking_amount
            ) 
        values ($1, $2, $3, $4, $5, $6, $7)
        returning *",
        &guest.id,
        3,
        &body.checkin_date,
        &body.checkout_date,
        &body.num_adults,
        &body.num_children,
        &body.booking_amount
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(booking) => {
            let booking_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "booking": booking
            })});

            return Ok((StatusCode::CREATED, Json(booking_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"status": "error", "message": format!("{:?}", e)})),
            ));
        }
    }
}

// Handler to update a booking for the guest
pub async fn update_booking_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(guest): Extension<Guest>,
    Json(body): Json<UpdateBookingSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        Booking,
        "select * from booking where id = $1 and guest_id = $2",
        id,
        &guest.id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Booking with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let booking = query_result.unwrap();

    let query_result = sqlx::query_as!(
        Booking,
        "update booking set 
        checkin_date = $1, 
        checkout_date = $2, 
        num_adults = $3, 
        num_children = $4, 
        booking_amount = $5, 
        updated_at = $6 
        where id = $7 and guest_id = $8 
        returning *",
        body.checkin_date.unwrap_or(booking.checkin_date),
        body.checkout_date.unwrap_or(booking.checkout_date),
        body.num_adults.unwrap_or(booking.num_adults),
        body.num_children.unwrap_or(booking.num_children),
        body.booking_amount.unwrap_or(booking.booking_amount),
        now,
        id,
        &guest.id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(booking) => {
            let booking_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "booking": booking
            })});
            return Ok(Json(booking_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"status": "error", "message": format!("{:?}", err)})),
            ))
        }
    }
}

// Handler to delete a booking for the guest
pub async fn delete_booking_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
    Extension(guest): Extension<Guest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!(
        "delete from booking where id = $1 and guest_id = $2",
        id,
        &guest.id
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Booking with ID: {} not found", id),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
