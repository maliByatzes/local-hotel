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

// Handler to register the guest using the parsed json body
pub async fn register_guest_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterGuestSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check from the database if the supplied email already exists
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

    // If the email address is found, the guest is not allowed to register
    if let Some(exists) = guest_exists {
        if exists {
            // Return error_response in json with status code
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Guest with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    // TODO: Check for duplicate phone number

    // Generate a random salt for password hashing
    let salt = SaltString::generate(&mut OsRng);
    // Generate hashed_password using argon2 default algorithm
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            // Return error_response in json with status code if there was an error
            // hashing the password
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    // Execute a SQL query to database inserting a new guest
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

    // Construct a json response of success containing the guest data
    let guest_response = serde_json::json!({"status": "success", "data": serde_json::json!({
        "guest": filter_guest_record(&guest)
    })});

    // Return the json response
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

// Handler to login the guest using parsed json data
pub async fn login_guest_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginGuestSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Execute a SQL query to fetch a guest with the supplied email address
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

    // Verify the password in the json data with the hashed password in the database
    let is_valid = match PasswordHash::new(&guest.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    // Return an error_response if the passwords don't match
    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email and password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    // Set up TokenClaims
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: guest.id.to_string(),
        iat,
        exp,
    };

    // Construct a token with token claims
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    // Store the newly created token in a cookie
    // Configure settings of the cookie
    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    // Construct a response to return to client
    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());

    // Append the cookie to the response
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

// Handler to log out the guest
pub async fn logout_handle() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Delete th cookie with `token` name by making it expire
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    // Construct a response to return to client
    let mut response = Response::new(json!({"status": "success"}).to_string());

    // Append the expired cookie to the response
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

// Procteted handler to be accessed by a guest with access
pub async fn get_me_handler(
    Extension(guest): Extension<Guest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "guest": filter_guest_record(&guest)
        })
    });

    Ok(Json(json_response))
}

// Util function to filter the guest record to hide sensitive data
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
