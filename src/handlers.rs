use axum::{response::IntoResponse, Json};

// Configure health check handler
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Local Hotel API is alive and well";

    let json_resp = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_resp)
}
