use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub struct EngineHttpConfig {
    pub enable_metrics: bool,
}

#[derive(Clone)]
pub struct EngineState {
    pub wasm_cfg: engine_exec_wasm::ExecConfig,
}

#[derive(Deserialize)]
struct RunWasmReq {
    wasm_b64: String,
    input: serde_json::Value,
}

#[derive(Serialize)]
struct RunWasmResp {
    output: serde_json::Value,
    meta: serde_json::Value,
}

async fn run_wasm_handler(
    Extension(state): Extension<Arc<EngineState>>,
    Json(req): Json<RunWasmReq>,
) -> Result<Json<RunWasmResp>, axum::http::StatusCode> {
    let wasm_bytes = general_purpose::STANDARD
        .decode(req.wasm_b64.as_bytes())
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let exec = engine_exec_wasm::WasmExecutor::new(state.wasm_cfg.clone())
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let out_bytes = exec
        .exec(&wasm_bytes, &req.input)
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let output: serde_json::Value = serde_json::from_slice(&out_bytes)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let meta = serde_json::json!({
        "fuel_limit": state.wasm_cfg.fuel_limit,
        "memory_limit_bytes": state.wasm_cfg.memory_limit_bytes,
        "deterministic": true
    });

    Ok(Json(RunWasmResp { output, meta }))
}

pub fn engine_router(cfg: EngineHttpConfig) -> Router {
    let mut router = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ready", get(|| async { "ok" }))
        .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
        .layer(TraceLayer::new_for_http());

    if cfg.enable_metrics {
        router = router.route("/metrics", get(|| async { "# HELP engine 1\nengine 1\n" }));
    }

    router
}

pub fn engine_router_with_wasm(cfg: EngineHttpConfig) -> Router {
    let state = Arc::new(EngineState {
        wasm_cfg: engine_exec_wasm::ExecConfig::default(),
    });

    let mut router = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ready", get(|| async { "ok" }))
        .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
        .route("/run-wasm", post(run_wasm_handler))
        .layer(TraceLayer::new_for_http());

    if cfg.enable_metrics {
        router = router.route("/metrics", get(|| async { "# HELP engine 1\nengine 1\n" }));
    }

    router.layer(axum::Extension(state))
}
