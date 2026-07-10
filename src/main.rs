mod api;
mod config;
mod entities;
mod error;
mod oauth;
use crate::{api::api_router, config::Config};

use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Clone, Debug)]
struct AppState {
    pub env: Config,
    pub conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::init();

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("tower_http=debug".parse().unwrap());
    fmt().with_env_filter(env_filter).with_target(false).init();

    let conn = Database::connect(&config.db_url)
        .await
        .expect("Database connection failed");

    let state = AppState {
        env: config.clone(),
        conn,
    };

    let listener = tokio::net::TcpListener::bind(&config.server_url)
        .await
        .unwrap();
    tracing::info!("Server starting at {}", &config.server_url);

    let router = Router::new()
        .nest("/api/v1", api_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    axum::serve(listener, router).await.unwrap();
}
