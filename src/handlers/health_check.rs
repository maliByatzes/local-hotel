use axum::{http::StatusCode, response::IntoResponse, Json};

// Configure health check handler
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Local Hotel API is alive and well";

    let json_resp = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_resp)
}

// Configure not found handler
pub async fn handler_404() -> impl IntoResponse {
    const MESSAGE: &str = "Content not found";

    let json_resp = serde_json::json!({
        "status": "fail",
        "message": MESSAGE
    });

    (StatusCode::NOT_FOUND, Json(json_resp))
}
