mod handlers;

use std::{env, time::Duration};

use axum::{routing::get, Router};
use handlers::health_check_handler;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // Get the connection string
    let db_connection_str = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://root:secret@localhost:5432/local_hotel".to_string());

    // Set up the database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("cannot connect to database");

    // Configure routing with application
    // Add database to the app
    let app = Router::new()
        .route("/health", get(health_check_handler))
        .with_state(db_pool);

    // Run app with tokio rt
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
