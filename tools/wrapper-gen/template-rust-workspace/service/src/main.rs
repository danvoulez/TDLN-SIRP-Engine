use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod routes; mod config; mod error; mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::default();
    let state = Arc::new(state::AppState::default());

    let api = routes::api::v1::router(state.clone());
    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/ready", get(|| async { axum::Json(serde_json::json!({"ok": true})) }))
        .nest("/v1", api)
        .layer(CorsLayer::permissive())
        .layer(routes::middleware::logging::layer());

    let addr = SocketAddr::from(([0,0,0,0], cfg.service_port));
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
