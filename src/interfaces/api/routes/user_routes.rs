use axum::{routing::post, Router, Json};
use crate::application::services::UserService;
use serde::{Deserialize, Serialize};

pub fn user_routes() -> Router {
    Router::new()
        .route("/register", post(register_user))
}

#[derive(Deserialize, Serialize)]
struct RegisterUser {
    username: String,
    password: String,
}

async fn register_user(Json(payload): Json<RegisterUser>) -> &'static str {
    let password_hash = hash_password(&payload.password); // Пример хеширования пароля
    let user = UserService::create_user(0, payload.username, password_hash);
    "User registered!"
}

fn hash_password(password: &str) -> String {
    password.to_string()
}