use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;

use crate::{
    models::Guest,
    response::FilteredGuest,
    schema::{LoginGuestSchema, RegisterGuestSchema, TokenClaims},
    AppState,
};

pub async fn register_guest_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterGuestSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let guest_exists: Option<bool> =
        sqlx::query_scalar("select exists(select 1 from guest where email_address = $1)")
            .bind(body.email_address.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e)
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = guest_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Guest with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    // TODO: Check for duplicate phone number

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                 "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let guest = sqlx::query_as!(
        Guest,
        "insert into guest (first_name, last_name, email_address, password, phone_number) values ($1, $2, $3, $4, $5) returning *",
        body.first_name.to_string(),
        body.last_name.to_string(),
        body.email_address.to_string(),
        hashed_password,
        body.phone_number.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let guest_response = serde_json::json!({"status": "success", "data": serde_json::json!({
        "guest": filter_guest_record(&guest)
    })});

    Ok(Json(guest_response))
}

// Configure health check handler
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Local Hotel API is alive and well";

    let json_resp = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_resp)
}

pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginGuestSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let guest = sqlx::query_as!(
        Guest,
        "select * from guest where email_address = $1",
        body.email_address.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&guest.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email and password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: guest.id.to_string(),
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn logout_handle() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success"}).to_string());

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

fn filter_guest_record(guest: &Guest) -> FilteredGuest {
    FilteredGuest {
        id: guest.id,
        first_name: guest.first_name.to_owned(),
        last_name: guest.last_name.to_owned(),
        email_address: guest.email_address.to_owned(),
        verified: guest.verified,
        phone_number: guest.phone_number.to_owned(),
        created_at: guest.created_at.unwrap(),
        updated_at: guest.updated_at.unwrap(),
    }
}
