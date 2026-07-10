pub mod auth;
pub mod user;

use crate::AppState;
use crate::api::user::user_router;

// use axum::middleware;
use axum::{Router, http::StatusCode, response::IntoResponse};

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", axum::routing::get(health_handler))
        .nest("/user", user_router())
        .fallback(handler_404)
}

async fn health_handler() -> impl IntoResponse {
    "Service is operational and healthy."
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
