use axum::{Router, routing::get};
use dotenv::dotenv;

mod infrastructure;
mod interfaces;
mod application;
mod domain;

#[deny(dead_code)]
#[forbid(unsafe_code)]
#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .merge(interfaces::api::routes::user_routes::user_routes())
        .merge(interfaces::api::routes::task_routes::task_routes())
        .route("/health", get(interfaces::http::handlers::health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}