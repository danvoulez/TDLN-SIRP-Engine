use crate::state::AppState;
use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tdln_certified_runtime::{CertifiedRuntime, RuntimeConfig};
use tdln_runtime_wasm::WasmCertifiedRuntime;

#[derive(Deserialize)]
pub struct RunReq {
    pub data: Value,
}

pub async fn run(State(_st): State<Arc<AppState>>, Json(req): Json<RunReq>) -> Json<Value> {
    // Minimal deterministic demo unit (alloc/dealloc/run ABI):
    // echoes the input JSON bytes back as output.
    let demo_unit_wat = r#"
      (module
        (memory (export "memory") 1)
        (global $heap (mut i32) (i32.const 1024))
        (func $alloc (export "alloc") (param $len i32) (result i32)
          (local $ptr i32)
          (local.set $ptr (global.get $heap))
          (global.set $heap (i32.add (global.get $heap) (local.get $len)))
          (local.get $ptr))
        (func (export "dealloc") (param i32 i32))
        (func (export "run") (param $ptr i32) (param $len i32) (result i32 i32)
          (local $out i32)
          (local.set $out (call $alloc (local.get $len)))
          (memory.copy (local.get $out) (local.get $ptr) (local.get $len))
          (local.get $out)
          (local.get $len)))
    "#;

    let unit_bytes = match wat::parse_str(demo_unit_wat) {
        Ok(b) => b,
        Err(e) => {
            return Json(serde_json::json!({
              "kind": "tdln.poi.v1",
              "error": format!("failed_to_build_demo_unit: {e}")
            }))
        }
    };

    let cfg = RuntimeConfig {
        deterministic: true,
        fuel: 10_000_000,
        memory_max_mb: 64,
    };
    let rt = WasmCertifiedRuntime {
        version: env!("CARGO_PKG_VERSION"),
    };
    match rt.execute(&unit_bytes, &req.data, &cfg) {
        Ok(card) => Json(serde_json::to_value(card).unwrap()),
        Err(e) => Json(serde_json::json!({
          "kind": "tdln.poi.v1",
          "error": format!("runtime_execute_failed: {e}")
        })),
    }
}
