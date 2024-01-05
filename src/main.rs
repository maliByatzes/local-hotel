mod config;
mod handlers;
mod jwt_auth;
mod models;
mod response;
mod schema;

use std::{sync::Arc, time::Duration};

use axum::{routing::get, Router};
use config::Config;
use dotenv::dotenv;
use handlers::health_check_handler;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    // Set up the database connection pool
    let db_pool = match PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
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
        env: config.clone(),
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
