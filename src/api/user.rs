use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    })
}

pub fn user_router() -> Router {
    Router::new().route("/", get(get_user))
}
