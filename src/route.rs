use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{
        get_me_handler, health_check_handler, login_guest_handler, logout_handle,
        register_guest_handler,
    },
    jwt_auth::auth,
    AppState,
};

// Construct a new router with all the paths
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_check_handler))
        .route("/v1/api/auth/register", post(register_guest_handler))
        .route("/v1/api/auth/login", post(login_guest_handler))
        .route(
            "/v1/api/auth/logout",
            get(logout_handle).route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/v1/api/guests/me",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}