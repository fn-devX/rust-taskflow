use axum::{Router, routing::get};
use dotenv::dotenv;
use std::net::SocketAddr;

mod infrastructure;
mod interfaces;
mod application;
mod domain;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .merge(interfaces::api::user_routes())
        .route("/health", get(interfaces::http::handlers::health_check));  // Проверка статуса API

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}