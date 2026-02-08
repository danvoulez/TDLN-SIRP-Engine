use axum::{http::StatusCode, Json};

pub async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "ok")
}
pub async fn ready() -> Json<serde_json::Value> {
    Json(serde_json::json!({"ok": true}))
}
