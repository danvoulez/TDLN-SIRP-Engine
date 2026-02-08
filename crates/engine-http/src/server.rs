
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::signer;
tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use axum::{routing::{get, post}, Router, Json};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use axum::extract::State;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_core::model::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_core::runtime::Engine;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_core::providers::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_extras::aggregator_kofn::KOfN;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_extras::expr_registry::{ExtensibleExpr, BasicRegistry};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_extras::sink_filesystem::FsSink;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_registry::file_registry::FileRegistry;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_registry::schema::EngineRegistryEntry;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::presign::{Presigner, PresignIntent, PresignResponse, StubPresigner};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_loader::{UnitStore, load_units_from_dir, watch_units};

#[derive(Clone)]
pub struct AppState<P: Presigner> {
    pub units: UnitStore,
    pub engine: Engine<UlidGen, crate::DefaultExprWrap, KOfN, DefaultCanon, DefaultCid, NoopSigner, FsSink>,
    pub k: usize,
    pub reg: FileRegistry,
    pub presigner: std::sync::Arc<P>,
}
pub struct DefaultExprWrap;
impl Default for DefaultExprWrap { fn default()->Self { Self } }
impl ExprEval for DefaultExprWrap {
    fn eval(&self, e:&Expression, ctx:&serde_json::Value)->anyhow::Result<serde_json::Value> {
        crate::DefaultExprShim.eval(e, ctx)
    }
}
// shim to reuse once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use DefaultExpr without bringing type params into state
struct DefaultExprShim;
impl DefaultExprShim {
    fn eval(&self, expr:&Expression, ctx:&serde_json::Value)->anyhow::Result<serde_json::Value>{
        engine_core::providers::DefaultExpr{}.eval(expr, ctx)
    }
}

#[derive(Deserialize)]
pub struct RunBody {
    #[serde(default)]
    pub options: Option<RunOptions>,
    pub input: serde_json::Value,
    pub unit_ref: Option<String>,

    #[serde(default)]
    pub k: Option<usize>,
}
#[derive(Serialize)]
pub struct RunResp {
    pub receipt: engine_core::model::ExecutionReceipt,
    pub card: serde_json::Value,
}

pub async fn build_router<P: Presigner>(outdir:&str, regdir:&str, k:usize, presigner:P) -> Router {
    let policy_a = PolicyBit::new("has_role","actor has role")
        .requires(&["actor","role"])
        .condition(Expression::eq(Expression::context(&["actor","role"]), Expression::literal("admin"))).build();
    let policy_b = PolicyBit::new("has_quota","quota > 0")
        .requires(&["actor","quota"])
        .condition(Expression::gt(Expression::context(&["actor","quota"]), Expression::literal(0))).build();
    let policy_c = PolicyBit::new("resource_ok","resource not restricted")
        .requires(&["resource","restricted"])
        .condition(Expression::not(Expression::context(&["resource","restricted"]))).build();
    let units_dir = std::env::var("UNITS_DIR").ok();
    let store = UnitStore::new(units_dir.clone().unwrap_or_else(|| "./units".into()));
    if let Some(dir) = &units_dir { let _ = tokio::spawn(watch_units(store.clone())); }

    let default_unit = if let Some(dir) = &units_dir {
        if let Ok(list) = load_units_from_dir(dir).await { list.into_iter().next() }
        else { None }
    } else { None };

    let selected = default_unit;
let engine = Engine::default()
        .chips(store.list())
        .agg(KOfN{ k })
        .expr(ExtensibleExpr{ reg: BasicRegistry::new() })
        .sink(FsSink::new(outdir))
        .build();

    let state = AppState {
        engine, k, units: store.clone(),
        reg: FileRegistry::new(regdir),
        presigner: std::sync::Arc::new(presigner),
    };

    Router::new()
            .route("/v1/apps/register", post(register_app))
            .route("/r/:run", get(handle_run_cid))
        .route("/health", get(|| async { "ok" }))
        .route("/run", post(run::<P>))
        .route("/registry/put", post(registry_put::<P>))
        .route("/acquire_presigned_url", post(acquire_presigned_url::<P>))
        .with_state(state)
}

async fn run<P: Presigner>(State(state): State<AppState<P>>, Json(body): Json<RunBody>) -> Json<RunResp> {
    use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use engine_core::model::{EngineMode, CanonSlot, Decision, MissingInfo, Proof};
    let _k = body.k.unwrap_or(state.k);
    let input_json = body.input;

    // Enforce unit_ref: if missing, return ASK (Doubt) with PoI: missing unit_ref
    if body.unit_ref.is_none() {
        let poi = ProofOfIndecision::missing(vec!["unit_ref"]);
        let card = make_card(Decision::ASK, None, None, None, Some(poi, run_cid.clone(), extra_refs.clone()), false, None, vec![]);
        return let json = serde_json::to_value(&card).unwrap(); RUN_INDEX.lock().unwrap().insert(run_cid.clone(), json.clone()); Json(card).into_response();
    
        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let receipt = engine_core::model::ExecutionReceipt{
            chip_id: "MISSING_UNIT_REF".into(),
            chip_hash: "b3:missing".into(),
            mode: EngineMode::conservative(),
            input: CanonSlot{ raw: input_json.clone(), canon: input_json.clone(), cid: "b3:missing".into() },
            policy_decisions: Vec::new(),
            output: CanonSlot{ raw: serde_json::json!({}), canon: serde_json::json!({}), cid: "b3:missing".into() },
            decision: Decision::Doubt,
            missing: Some(MissingInfo{ id:"unit_ref".into(), reason:"required".into(), missing_fields: vec!["unit_ref".into()], missing_evidence: vec![], resolution_hint: Some("include unit_ref (CID or registry id)".into()) }),
            proof: Proof{ hash_chain: Vec::new(), signature: None },
            timestamp: now,
            duration_ns: 0,
        };
        let card = serde_json::json!({
            "kind":"receipt.card.v1",
            "realm":"trust",
            "decision":"ASK",
            "poi":{"present":true,"missing":["unit_ref"]},
            "proof": receipt.proof,
            "ts": receipt.timestamp
        });
        return Json(RunResp{ receipt, card });
    }

    let unit_id = body.unit_ref.unwrap();
    let receipt = state.engine.execute(&unit_id, input_json, None).expect("execute");
    let card = serde_json::json!({
        "kind":"receipt.card.v1",
        "realm":"trust",
        "unit_id": receipt.chip_id,
        "decision": match receipt.decision { Decision::Allow => "ACK", Decision::Deny => "NACK", _ => "ASK" },
        "input": { "cid": receipt.input.cid },
        "output": { "cid": receipt.output.cid },
        "proof": receipt.proof,
        "ts": receipt.timestamp
    });
    Json(RunResp{ receipt, card })
},
        "output": { "cid": receipt.output.cid },
        "proof": receipt.proof,
        "ts": receipt.timestamp
    });
    Json(RunResp{ receipt, card })
}

#[derive(Deserialize)]
struct RegPutBody { name:String, version:String, cid:String }
#[derive(Serialize)]
struct RegPutResp { path:String }
async fn registry_put<P: Presigner>(State(state): State<AppState<P>>, Json(b): Json<RegPutBody>) -> Json<RegPutResp> {
    let e = EngineRegistryEntry{
        kind:"engine.registry.entry.v1".into(),
        id: ulid::Ulid::new().to_string(),
        name: b.name, version: b.version, cid: b.cid, meta: serde_json::json!({})
    };
    let p = state.reg.put(&e).expect("registry put");
    Json(RegPutResp{ path: p.display().to_string() })
}

#[derive(Deserialize)]
struct PresignBody { actor:String, resource: crate::presign::PresignResource, ttl_seconds: u64 }
async fn acquire_presigned_url<P: Presigner>(State(state): State<AppState<P>>, Json(b): Json<PresignBody>) -> Json<PresignResponse> {
    let intent = PresignIntent{ actor: b.actor, resource: b.resource, ttl_seconds: b.ttl_seconds };
    let resp = state.presigner.presign(intent).await.expect("presign");
    Json(resp)
}


use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use axum::http::StatusCode;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use axum::extract::DefaultBodyLimit;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_canon::json_atomic_stringify;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use tdln_cid::blake3_cid;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use axum::routing::post as ax_post;

#[derive(Deserialize)]
pub struct SubmitCodeBody {
    pub actor: String,
    pub code: Option<String>,         // inline code (small)
    pub url: Option<String>,          // repository or zip URL (placeholder)
    pub meta: Option<serde_json::Value>,
}
#[derive(Deserialize)]
pub struct SubmitDataBody {
    pub actor: String,
    pub data: serde_json::Value,      // JSON ✯ Atomic payload
    pub meta: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct SubmitResp {
    pub receipt: engine_core::model::ExecutionReceipt,
    pub card: serde_json::Value,
}

pub async fn build_router_with_flavors<P: Presigner>(outdir:&str, regdir:&str, k:usize, presigner:P) -> Router {
    let base = build_router::<P>(outdir, regdir, k, presigner).await;
    // Allow larger bodies for code/data submit (adjust as needed)
    base.layer(DefaultBodyLimit::max(16 * 1024 * 1024))
        .route("/submit-code", ax_post(submit_code::<P>))
        .route("/submit-data", ax_post(submit_data::<P>))
}

async fn submit_code<P: Presigner>(State(state): State<AppState<P>>, Json(b): Json<SubmitCodeBody>) -> Result<Json<SubmitResp>, StatusCode> {
    // Minimal placeholder: treat "code" as context for the same example unit.
    let input = serde_json::json!({
        "actor": { "role": "admin", "quota": 5, "id": b.actor },
        "resource": { "restricted": false },
        "artifact": { "kind":"code", "present": b.code.is_some() || b.url.is_some(), "meta": b.meta }
    });
    let receipt = state.engine.execute("", input, None).map_err(|_| StatusCode::BAD_REQUEST)?;
    let card = serde_json::json!({
        "kind":"receipt.card.v1",
        "unit_id": receipt.chip_id,
        "decision": receipt.decision,
        "input": { "cid": receipt.input.cid },
        "output": { "cid": receipt.output.cid },
        "proof": receipt.proof,
        "ts": receipt.timestamp
    });
    Ok(Json(SubmitResp{ receipt, card }))
}

async fn submit_data<P: Presigner>(State(state): State<AppState<P>>, Json(b): Json<SubmitDataBody>) -> Result<Json<SubmitResp>, StatusCode> {
    let input = serde_json::json!({
        "actor": { "role": "admin", "quota": 5, "id": b.actor },
        "resource": { "restricted": false },
        "payload": b.data,
        "meta": b.meta
    });
    let receipt = state.engine.execute("", input, None).map_err(|_| StatusCode::BAD_REQUEST)?;
    let card = serde_json::json!({
        "kind":"receipt.card.v1",
        "unit_id": receipt.chip_id,
        "decision": receipt.decision,
        "input": { "cid": receipt.input.cid },
        "output": { "cid": receipt.output.cid },
        "proof": receipt.proof,
        "ts": receipt.timestamp
    });
    Ok(Json(SubmitResp{ receipt, card }))
}


#[derive(serde::Deserialize, Default)]
pub struct RunOptions {
    #[serde(default)]
    pub require_certified_runtime: bool,
    #[serde(default)]
    pub offline_bundle: bool,
    #[serde(default)]
    pub no_hitl: bool,
}


fn compute_run_cid(unit_ref: &str, realm: Option<&str>, input: &serde_json::Value, opts: &RunOptions) -> String {
    let manifest = json!({
        "kind":"run.manifest.v1",
        "unit_ref": unit_ref,
        "realm": realm.unwrap_or("trust"),
        "input": input,
        "options": {
            "no_hitl": opts.no_hitl,
            "offline_bundle": opts.offline_bundle,
            "require_certified_runtime": opts.require_certified_runtime
        }
    });
    let canonical = json_atomic_stringify(&manifest);
    blake3_cid(&canonical)
}

fn compute_run_cid_minimal(body: &RunBody) -> String {
    let manifest = json!({
        "kind":"run.manifest.v1",
        "unit_ref": null,
        "realm": body.realm.as_deref().unwrap_or("trust"),
        "input": body.input,
        "options": {}
    });
    let canonical = json_atomic_stringify(&manifest);
    blake3_cid(&canonical)
}

fn resolve_url(run_cid: &str) -> String {
    format!("https://cert.tdln.foundry/r/{}", run_cid)
}


fn sirp_capsule_intent(from: &str, to: &str, run_cid: &str) -> serde_json::Value {
    json!({
      "kind":"sirp.capsule.v1",
      "type":"INTENT",
      "from": from,
      "to": to,
      "ts": chrono::Utc::now().to_rfc3339(),
      "refs":[{"kind":"run.manifest","cid": run_cid}],
      "aad":{},
      "signature": signer::sign_bytes(&cap_canon.as_bytes()) // assinatura real deve ser aplicada no signer do engine
    })
}
fn sirp_capsule_result(from: &str, to: &str, card_cid: &str) -> serde_json::Value {
    json!({
      "kind":"sirp.capsule.v1",
      "type":"RESULT",
      "from": from,
      "to": to,
      "ts": chrono::Utc::now().to_rfc3339(),
      "refs":[{"kind":"receipt.card","cid": card_cid}],
      "aad":{},
      "signature": signer::sign_bytes(&cap_canon.as_bytes())
    })
}
fn sirp_delivery(capsule_cid: &str, sender_did: &str, receiver_did: &str, outcome: &str) -> serde_json::Value {
    json!({
      "kind":"sirp.receipt.delivery.v1",
      "capsule_cid": capsule_cid,
      "sender_did": sender_did,
      "receiver_did": receiver_did,
      "ts_received": chrono::Utc::now().to_rfc3339(),
      "outcome": outcome,
      "signature": signer::sign_bytes(&cap_canon.as_bytes())
    })
}
fn sirp_execution(capsule_cid: &str, executor_did: &str, card_cid: &str, runtime_used: bool, eer_cid: Option<&str>) -> serde_json::Value {
    json!({
      "kind":"sirp.receipt.execution.v1",
      "capsule_cid": capsule_cid,
      "executor_did": executor_did,
      "ts_done": chrono::Utc::now().to_rfc3339(),
      "result_cid": card_cid,
      "runtime_used": runtime_used,
      "eer_cid": eer_cid.unwrap_or(""),
      "signature": signer::sign_bytes(&cap_canon.as_bytes())
    })
}

fn emit_sirp_intent_and_delivery(run_cid: &str) -> (String, String) {
    let from = "did:tdln:issuer:m1";
    let to = "did:tdln:engine:exec";
    let cap = sirp_capsule_intent(from, to, run_cid);
    let cap_canon = json_atomic_stringify(&cap);
    let cap_cid = blake3_cid(&cap_canon);
    let del = sirp_delivery(&cap_cid, from, to, "DELIVERED");
    let del_canon = json_atomic_stringify(&del);
    let del_cid = blake3_cid(&del_canon);
    (cap_cid, del_cid)
}

fn emit_sirp_result_and_execution(card_payload: &serde_json::Value, runtime_used: bool, eer_cid: Option<&str>, intent_cid: &str) -> (String, String) {
    let card_canon = json_atomic_stringify(card_payload);
    let card_cid = blake3_cid(&card_canon);
    let from = "did:tdln:engine:exec";
    let to = "did:tdln:issuer:m1";
    let cap_res = sirp_capsule_result(from, to, &card_cid);
    let cap_res_canon = json_atomic_stringify(&cap_res);
    let cap_res_cid = blake3_cid(&cap_res_canon);
    let exe = sirp_execution(intent_cid, from, &card_cid, runtime_used, eer_cid);
    let exe_canon = json_atomic_stringify(&exe);
    let exe_cid = blake3_cid(&exe_canon);
    (cap_res_cid, exe_cid)
}

fn ref_cid(kind: &str, cid: &str) -> RefLink {
    RefLink{ kind: kind.into(), cid: cid.into(), media_type: None, hrefs: vec![] }
}


static RUN_INDEX: Lazy<Mutex<HashMap<String, serde_json::Value>>> = Lazy::new(|| Mutex::new(HashMap::new()));


use axum::{extract::Path, http::{HeaderMap, StatusCode}, response::{IntoResponse, Redirect}};

pub async fn handle_run_cid(Path(run): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    let wants_json = headers.get(axum::http::header::ACCEPT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.contains("application/json"))
        .unwrap_or(false);

    let store = RUN_INDEX.lock().unwrap();
    if let Some(card) = store.get(&run) {
        if wants_json {
            return (StatusCode::OK, axum::Json(card.clone())).into_response();
        } else {
            // best-effort pull realm/did; if absent, redirect to a generic UI route
            let realm = card.get("realm").and_then(|v| v.as_str()).unwrap_or("trust");
            let did = card.get("did").and_then(|v| v.as_str()).unwrap_or("did:tdln:unknown");
            let loc = format!("/{}/{}#{}", realm, did, run);
            return Redirect::to(&loc).into_response();
        }
    }
    (StatusCode::NOT_FOUND, "run not found").into_response()
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct AppRegEntry {
    name: String,
    did: String,
    pubkey_b64: String,
}
static APP_REG: Lazy<std::sync::Mutex<HashMap<String, AppRegEntry>>> = Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

#[derive(serde::Deserialize)]
struct RegisterBody { name: String, did: String, pubkey_b64: String }

pub async fn register_app(axum::Json(body): axum::Json<RegisterBody>) -> impl axum::response::IntoResponse {
    let mut map = APP_REG.lock().unwrap();
    map.insert(body.did.clone(), AppRegEntry { name: body.name.clone(), did: body.did.clone(), pubkey_b64: body.pubkey_b64.clone() });
    (axum::http::StatusCode::CREATED, "registered")
}

fn verify_app_signature(did_opt: Option<&str>, sig_opt: Option<&str>, canon_body: &str) -> (bool, String) {
    let did = match did_opt { Some(v) if !v.is_empty() => v.to_string(), _ => return (false, "no_did".into()) };
    let sig = match sig_opt { Some(v) if v.starts_with("ed25519:") => v["ed25519:".len()..].to_string(), _ => return (false, "no_sig".into()) };
    let reg = APP_REG.lock().unwrap();
    let app = match reg.get(&did) { Some(a) => a, None => return (false, "unknown_did".into()) };
    let pk_b = match base64::decode(&app.pubkey_b64) { Ok(b) => b, Err(_) => return (false, "bad_pub".into()) };
    let sig_b = match base64::decode(&sig) { Ok(b) => b, Err(_) => return (false, "bad_sig".into()) };
    use ed25519_dalek::{Verifier, VerifyingKey, Signature};
    let Ok(vk) = VerifyingKey::from_bytes(pk_b.as_slice().try_into().unwrap_or(&[0u8;32])) else { return (false, "pk_len".into()) };
    let Ok(signature) = Signature::from_slice(&sig_b) else { return (false, "sig_len".into()) };
    match vk.verify(canon_body.as_bytes(), &signature) {
        Ok(_) => (true, "ok".into()),
        Err(_) => (false, "verify_fail".into())
    }
}
