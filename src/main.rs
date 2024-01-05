mod config;
mod handlers;
mod models;
mod schema;

use std::{env, sync::Arc, time::Duration};

use axum::{routing::get, Router};
use dotenv::dotenv;
use handlers::health_check_handler;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Get the connection string
    let db_connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Set up the database connection pool
    let db_pool = match PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌Failed connection to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Init app state
    let app_state = Arc::new(AppState {
        db: db_pool.clone(),
    });

    // Configure routing with application
    // Add database to the app
    let app = Router::new()
        .route("/health", get(health_check_handler))
        .with_state(app_state);

    println!("🚀 Server started successfully, on port 3000");
    // Run app with tokio rt
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
