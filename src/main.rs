mod api;
mod oauth;
use crate::api::api_router;

use axum::Router;
use std::env;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("tower_http=debug".parse().unwrap());
    fmt().with_env_filter(env_filter).with_target(false).init();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let server_url = format!("{host}:{port}");

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    tracing::info!("Server starting at {}", server_url);

    let router = Router::new().nest("/api/v1", api_router()).layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    axum::serve(listener, router).await.unwrap();
}
