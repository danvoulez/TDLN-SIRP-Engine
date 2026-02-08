use axum::{Json, extract::State};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RunReq { pub data: Value }

pub async fn run(State(_st): State<Arc<AppState>>, Json(req): Json<RunReq>) -> Json<Value> {
    // Stub: engine call would happen here; emit a synthetic receipt-like card
    Json(serde_json::json!({
      "kind": "tdln.card.v1",
      "decision": "ACK",
      "proof": { "seal": {"alg":"ed25519-blake3", "kid":"local", "sig":"DEMO"}},
      "links": { "card_url": "https://cert.tdln.foundry/r/b3:DEMO" },
      "input_hash": format!("b3:{:x}", blake3::hash(serde_json::to_string(&req.data).unwrap().as_bytes()))
    }))
}
