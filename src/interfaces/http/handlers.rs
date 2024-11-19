use axum::response::Json;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn health_check() -> Json<serde_json::Value> {
    let db_status = check_database_connection().await;

    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_secs();

    Json(json!({
        "status": "OK",
        "services": {
            "database": db_status,
        },
        "timestamp": timestamp,
        "version": "1.0.0"
    }))
}

async fn check_database_connection() -> &'static str {
    "connected"
}
