use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Configure routing with application
    let app = Router::new().route("/health", get(health_check_handler));

    // Run app with tokio rt
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Configure health check handler
async fn health_check_handler() -> &'static str {
    "Local Hotel is alive and kicking"
}
