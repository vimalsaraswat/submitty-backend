pub mod user;
use crate::api::user::user_router;

use axum::{Router, response::IntoResponse};

async fn health_handler() -> impl IntoResponse {
    "Service is operational and healthy."
}

pub fn api_router() -> Router {
    Router::new()
        .route("/health", axum::routing::get(health_handler))
        .nest("/user", user_router())
}
