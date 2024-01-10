// Import modules
mod config;
mod handlers;
mod jwt_auth;
mod models;
mod response;
mod route;
mod schema;

use std::{sync::Arc, time::Duration};

use crate::route::create_router;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::Config;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Keep track of shared elements in the AppState structure
pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

// Use tokio runtime to make the main function async
#[tokio::main]
async fn main() {
    // Check if the `.env` is available form the root directory
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "local_hotel=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Init config from config.rs with the environment variables
    let config = Config::init();

    // Set up the database connection pool,
    // with 10 max connections
    // and connection timeout of 3 seconds
    let db_pool = match PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            tracing::debug!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            tracing::error!("❌Failed connection to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Run db migrations
    sqlx::migrate!().run(&db_pool).await.unwrap();

    // Init cors with different configurations
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Init app state
    let app_state = Arc::new(AppState {
        db: db_pool.clone(),
        env: config.clone(),
    });

    // Configure routing with application
    // Add database to the app
    let app = create_router(app_state).layer(cors);

    // Run app with tokio rt
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("🚀 Server started successfully, on port 3000");
    axum::serve(listener, app).await.unwrap();
}
