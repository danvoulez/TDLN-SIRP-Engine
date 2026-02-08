
use engine_http::server::build_router_with_flavors;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use std::time::{Duration, Instant};
use axum::{response::IntoResponse, http::StatusCode, routing::get, Router};
mod config;
use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;
use engine_http::presign::StubPresigner;

#[tokio::main]
async fn main() {
    let cfg = config::AppConfig::from_env();
    let api = build_router_with_flavors::<StubPresigner>("./out", "./registry", 2, StubPresigner).await;
    let static_dir = ServeDir::new("static");
    let mut cors = CorsLayer::permissive();
    if cfg.cors_origins.len() == 1 && cfg.cors_origins[0] == "*" { cors = CorsLayer::permissive(); }
    let app = api
        .nest_service("/", get_service(static_dir))
        .route("/ready", get(|| async { (StatusCode::OK, "ready") }))
        .route("/version", get(|| async { (StatusCode::OK, env!("CARGO_PKG_VERSION")) }))
        .route("/metrics", get(|| async { (
            StatusCode::OK,
            [("content-type","text/plain; version=0.0.4")],
            format!(
                "# HELP wrapper_build_info build info\n# TYPE wrapper_build_info gauge\nwrapper_build_info{{name=\"{{WRAPPER_NAME}}\",preset=\"{{PRESET}}\",flavors=\"{{FLAVORS}}\"}} 1\n"
            )
        ) }))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .fallback(|| async { (StatusCode::NOT_FOUND, axum::Json(serde_json::json!({"error":"not_found"}))) });
    let addr = format!("0.0.0.0:{}", cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 {{WRAPPER_NAME}} running on http://0.0.0.0:{}  —  flavors: {{FLAVORS}}  —  color: {}  —  preset: {{PRESET}}", cfg.port, cfg.brand_color);
    axum::serve(listener, app).await.unwrap();
}
