use once_cell::sync::Lazy;

use axum::{Router, routing::{get, post}};
use tower_http::trace::TraceLayer;

pub struct EngineHttpConfig {
    pub enable_metrics: bool,
}

pub fn engine_router(cfg: EngineHttpConfig) -> Router {
    let mut r = Router::new().route("/.well-known/logline/grl.json", axum::routing::get(well_known_grl))
        .route("/health", get(|| async { "ok" }))
        .route("/ready", get(|| async { "ok" }))
        .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
        .route("/run", post(|| async { axum::Json(serde_json::json!({"receipt":"stub"})) }))
        .route("/registry/put", post(|| async { axum::Json(serde_json::json!({"ok":true})) }))
        .route("/acquire_presigned_url", post(|| async { axum::Json(serde_json::json!({"url":"stub"})) }))
        .layer(TraceLayer::new_for_http());
    if cfg.enable_metrics {
        r = r.route("/metrics", get(|| async { "# HELP engine 1\nengine 1\n" }));
    }
    r
}


use axum::{Json, extract::State};
use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};
use std::sync::Arc;

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

async fn run_wasm_handler(State(state): State<Arc<EngineState>>, Json(req): Json<RunWasmReq>) -> Result<Json<RunWasmResp>, axum::http::StatusCode> {
    let wasm_bytes = general_purpose::STANDARD
        .decode(req.wasm_b64.as_bytes())
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let exec = match engine_exec_wasm::WasmExecutor::new(state.wasm_cfg.clone()) {
        Ok(e) => e,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    };
    let out_bytes = exec.exec(&wasm_bytes, &req.input).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let output: serde_json::Value = serde_json::from_slice(&out_bytes).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let meta = serde_json::json!({
        "fuel_limit": state.wasm_cfg.fuel_limit,
        "memory_limit_bytes": state.wasm_cfg.memory_limit_bytes,
        "deterministic": true
    });
    Ok(Json(RunWasmResp{ output, meta }))
}

}

/// Build a router with deterministic WASM enabled (default limits).
pub fn engine_router_with_wasm(cfg: EngineHttpConfig) -> Router {
    use axum::Router;
    use std::sync::Arc;
    let mut r = engine_router(cfg);
    let state = Arc::new(EngineState{ wasm_cfg: engine_exec_wasm::ExecConfig::default() });
    r = r.route("/run-wasm", post(run_wasm_handler)).with_state(state);
        r = r.route("/registry/presign", post(presign_handler));
        r = r.route("/s3/proxy", get(s3_proxy_handler));
    r
}


use chrono::Utc;
use ulid::Ulid;
use blake3;
use tokio::fs as async_fs;
use std::path::PathBuf;

async fn emit_audit_report(base_dir: &str, who: &str, input_json: &serde_json::Value, output_json: &serde_json::Value, meta: &serde_json::Value) -> Result<(), axum::http::StatusCode> {
    let audit_id = Ulid::new().to_string();
    let ts = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let in_bytes = serde_json::to_vec(input_json).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let out_bytes = serde_json::to_vec(output_json).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let in_b3 = format!("b3:{}", blake3::hash(&in_bytes).to_hex());
    let out_b3 = format!("b3:{}", blake3::hash(&out_bytes).to_hex());

    let report = serde_json::json!({
        "kind": "audit.report.v1",
        "audit_id": audit_id,
        "ts": ts,
        "who": who,
        "intent": { "name": "run_wasm" },
        "plan": { "engine": "wasm-deterministic@v1", "abi": "alloc/dealloc/run(ptr,len)->(ptr,len)" },
        "runtime": { "meta": meta },
        "proofs": { "inputs_digest": in_b3, "result_digest": out_b3 }
    });

    let day = ts[..10].replace("-", "/"); // YYYY/MM/DD-ish arte
    let path = PathBuf::from(base_dir).join("audit").join(day).join(format!("{}.json", report["audit_id"].as_str().unwrap()));
    if let Some(parent) = path.parent() { async_fs::create_dir_all(parent).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?; }
    let data = serde_json::to_vec_pretty(&report).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    async_fs::write(path, data).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct PresignReq {
    backend: String,
    bucket: String,
    key: String,
    verb: String,
    ttl_secs: u64,
    who: Option<String>,
    // constraints (optional)
    ip_hash: Option<String>,
    byte_range_max: Option<u64>
}

#[derive(Serialize)]
struct PresignResp {
    url: String,
    meta: serde_json::Value
}


#[derive(Serialize)]
struct Poi {
    reason: String,
    missing: Vec<String>,
    violations: Vec<String>,
    hints: Vec<String>,
}

#[derive(Serialize)]
#[serde(tag = "decision")]
enum PolicyDecision {
    ACK,
    ASK { poi: Poi },
    NACK { error: String },
}

fn eval_presign_policy(req: &PresignReq) -> PolicyDecision {
    // Hard limits (representing TDLN policy)
    let allowed_verbs = ["GET","PUT"];
    if !allowed_verbs.contains(&req.verb.as_str()) {
        return PolicyDecision::NACK { error: "verb_not_allowed".into() };
    }

    let mut violations = Vec::new();
    let mut missing = Vec::new();
    let mut hints = Vec::new();

    // TTL policy: <= 600s
    if req.ttl_secs > 600 {
        violations.push(format!("ttl_secs_exceeds_max:{}", req.ttl_secs));
        hints.push("use ttl_secs <= 600".into());
    }

    // Key/prefix policy: must not contain `..`
    if req.key.contains("..") {
        violations.push("key_invalid_path_traversal".into());
        hints.push("remove '..' from key".into());
    }

    // Byte range policy: <= 100 MB if provided
    if let Some(br) = req.byte_range_max {
        if br > 100 * 1024 * 1024 {
            violations.push(format!("byte_range_max_too_large:{}", br));
            hints.push("set byte_range_max <= 104857600".into());
        }
    }

    // Backend policy: restrict to fs/s3 only in this demo
    if req.backend != "fs" && req.backend != "s3" {
        violations.push("backend_not_allowed".into());
        hints.push("use backend 's3' or 'fs'".into());
    }

    if violations.is_empty() && missing.is_empty() {
        PolicyDecision::ACK
    } else {
        PolicyDecision::ASK { poi: Poi {
            reason: "constraints_need_adjustment".into(),
            missing,
            violations,
            hints
        }}
    }
}


use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature, SECRET_KEY_LENGTH, PUBLIC_KEY_LENGTH};
use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize, Deserialize, Clone)]
struct AccessGrant {
    kind: String,             // "access.grant.v1"
    grant_id: String,         // ULID
    sub: String,              // who
    resource: serde_json::Value, // {store,bucket,prefix,object,verbs,constraints}
    exp: String,
    iat: String,
    nonce: String,
    seal: GrantSeal
}

#[derive(Serialize, Deserialize, Clone)]
struct GrantSeal {
    alg: String,   // "ed25519-blake3"
    kid: String,   // "dev-2026-key-1"
    sig: String    // base64
}

fn sign_grant(mut grant: AccessGrant, sk_b64: &str, kid: &str) -> Result<AccessGrant, axum::http::StatusCode> {
    let sk_bytes = general_purpose::STANDARD.decode(sk_b64).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let sk = SigningKey::from_bytes(&sk_bytes.try_into().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?);
    let mut to_sign = grant.clone(); to_sign.seal.sig = "".into();
    let bytes = serde_json::to_vec(&to_sign).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let digest = blake3::hash(&bytes);
    let sig = sk.sign(digest.as_bytes());
    grant.seal = GrantSeal{ alg:"ed25519-blake3".into(), kid: kid.into(), sig: general_purpose::STANDARD.encode(sig.to_bytes()) };
    Ok(grant)
}

fn verify_grant(grant: &AccessGrant, pk_b64: &str) -> Result<(), axum::http::StatusCode> {
    if grant.kind != "access.grant.v1" { return Err(axum::http::StatusCode::UNAUTHORIZED); }
    let pk_bytes = general_purpose::STANDARD.decode(pk_b64).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let vk = VerifyingKey::from_bytes(&pk_bytes.try_into().map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let mut to_verify = grant.clone(); 
    let sig_b = general_purpose::STANDARD.decode(to_verify.seal.sig.as_bytes()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let sig = Signature::from_bytes(&sig_b.try_into().map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?);
    to_verify.seal.sig = "".into();
    let bytes = serde_json::to_vec(&to_verify).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let digest = blake3::hash(&bytes);
    vk.verify(digest.as_bytes(), &sig).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)
}


fn b64url_nopad(data: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}
fn b64url_decode(s: &str) -> Result<Vec<u8>, axum::http::StatusCode> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(s.as_bytes()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)
}

// Minimal PASETO v4.public (Ed25519, detached footer = kid)
// token = "v4.public." + b64url(msg || sig || footer_json)
#[derive(Serialize, Deserialize, Clone)]
struct PasetoGrant {
    token: String,        // v4.public....
    payload: AccessGrant  // same grant payload (for convenience/debug)
}

fn paseto_sign(grant: &AccessGrant, sk_b64: &str) -> Result<String, axum::http::StatusCode> {
    let sk_bytes = base64::engine::general_purpose::STANDARD.decode(sk_b64).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let sk = ed25519_dalek::SigningKey::from_bytes(&sk_bytes.try_into().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?);
    let msg = serde_json::to_vec(grant).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let digest = blake3::hash(&msg); // v4.public usa Ed25519 direto; aqui mantemos ed25519(blake3(msg)) por coerência com seal
    let sig  = sk.sign(digest.as_bytes());
    let body = [msg.as_slice(), sig.as_bytes()].concat();
    let token = format!("v4.public.{}", b64url_nopad(&body));
    Ok(token)
}

fn paseto_verify(token: &str, pk_b64: &str) -> Result<AccessGrant, axum::http::StatusCode> {
    let prefix = "v4.public.";
    if !token.starts_with(prefix) { return Err(axum::http::StatusCode::UNAUTHORIZED); }
    let body = b64url_decode(&token[prefix.len()..])?;
    if body.len() < 64 { return Err(axum::http::StatusCode::UNAUTHORIZED); }
    let (msg, sigb) = body.split_at(body.len()-64);
    let grant: AccessGrant = serde_json::from_slice(msg).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let vk_bytes = base64::engine::general_purpose::STANDARD.decode(pk_b64).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let vk = ed25519_dalek::VerifyingKey::from_bytes(&vk_bytes.try_into().map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let digest = blake3::hash(msg);
    let sig = ed25519_dalek::Signature::from_bytes(sigb.try_into().map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?);
    vk.verify(digest.as_bytes(), &sig).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    Ok(grant)
}



static GRL_CACHE: once_cell::sync::Lazy<std::sync::Mutex<(std::time::Instant, Vec<u8>)>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new((std::time::Instant::now(), Vec::new())));


fn bump_grl_merge_metric(applied: bool) {
    if !applied { return; }
    let p = std::path::Path::new("metrics").join("grl_merge_applied.count");
    if let Ok(prev) = std::fs::read_to_string(&p) {
        if let Ok(n) = prev.trim().parse::<u64>() {
            let _ = std::fs::write(&p, format!("{}", n+1));
            return;
        }
    }
    let _ = std::fs::create_dir_all("metrics");
    let _ = std::fs::write(&p, "1");
}

async fn well_known_grl() -> Result<impl IntoResponse, axum::http::StatusCode> {
    // 1) Start from local manifest if exists
    let local_path = std::path::Path::new("revoked_grants").join("manifest.json");
    let mut merged: serde_json::Value = if local_path.exists() {
        let data = std::fs::read(&local_path).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        serde_json::from_slice(&data).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        serde_json::json!({ "kind":"revocation.manifest.v1", "updated_at": chrono::Utc::now().to_rfc3339(), "grants": [], "sig": "" })
    };

    // 2) Optional remote fetch + TTL cache
    if let Ok(remote) = std::env::var("GRL_REMOTE_URL") {
        let ttl_ms: u64 = std::env::var("GRL_TTL_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(60000);
        let mut guard = GRL_CACHE.lock().unwrap();
        let (last, buf) = &mut *guard;
        if buf.is_empty() || last.elapsed().as_millis() as u64 >= ttl_ms {
            drop(guard);
            // fetch
            let client = reqwest::Client::new();
            if let Ok(resp) = client.get(remote).send().await {
                if let Ok(bytes) = resp.bytes().await {
                    let mut g = GRL_CACHE.lock().unwrap();
                    *g = (std::time::Instant::now(), bytes.to_vec());
                }
            }
        }
        let (_, data) = &*GRL_CACHE.lock().unwrap();
        if !data.is_empty() {
            if let Ok(remote_v) = serde_json::from_slice::<serde_json::Value>(data) {
                // naive merge: concatenate unique grant ids, prefer remote sig/updated_at if newer
                if let (Some(local_g), Some(remote_g)) = (merged.get_mut("grants"), remote_v.get("grants")) {
                    if let (Some(local_arr), Some(remote_arr)) = (local_g.as_array_mut(), remote_g.as_array()) {
                        for r in remote_arr {
                            if !local_arr.contains(r) { local_arr.push(r.clone()); }
                        }
                    }
                }
                if let Some(ru) = remote_v.get("updated_at").and_then(|x| x.as_str()) {
                    merged["updated_at"] = serde_json::Value::String(ru.to_string());
                }
                if let Some(rs) = remote_v.get("sig") {
                    merged["sig"] = rs.clone();
                }
            }
        }
    }

    let body = serde_json::to_vec(&merged).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(([(axum::http::header::CONTENT_TYPE, "application/json")], body))
}

async fn health_handler() -> Result<impl IntoResponse, axum::http::StatusCode> {
    let presign = std::env::var("PRESIGN_DISABLE").ok().as_deref() != Some("1");
    let proxy = std::env::var("PROXY_DISABLE").ok().as_deref() != Some("1");
    let status = serde_json::json!({
        "ok": true,
        "presign_enabled": presign,
        "proxy_enabled": proxy,
        "ts": chrono::Utc::now().to_rfc3339()
    });
    let _ = emit_audit_report("./tenants/_public", "health", &serde_json::json!({}), &status, &serde_json::json!({"intent":"health"})).await;
    Ok(([(axum::http::header::CONTENT_TYPE, "application/json")], serde_json::to_vec(&status).unwrap()))
}

async fn presign_handler(Json(req): Json<PresignReq>) -> Result<Json<PresignResp>, axum::http::StatusCode> {
    if std::env::var("PRESIGN_DISABLE").ok().as_deref() == Some("1") { return Err(axum::http::StatusCode::SERVICE_UNAVAILABLE); }

// 0) Evaluate policy with constraints
let pd = eval_presign_policy(&req);
match &pd {
    PolicyDecision::NACK { error } => {
        let meta = serde_json::json!({
            "backend": req.backend, "bucket": req.bucket, "key": req.key,
            "verb": req.verb, "ttl_secs": req.ttl_secs,
            "constraints": { "ip_hash": req.ip_hash, "byte_range_max": req.byte_range_max }
        });
        let out = serde_json::json!({ "decision":"NACK","error": error });
        let _ = emit_audit_report("./tenants/_public", req.who.as_deref().unwrap_or("anonymous"), &meta, &out, &serde_json::json!({"intent":"presign"})).await;
        return Ok(Json(PresignResp{ url: "".into(), meta: out }));
    },
    PolicyDecision::ASK { poi } => {
        // Emit audit with PoI; do NOT issue a url
        let meta = serde_json::json!({
            "backend": req.backend, "bucket": req.bucket, "key": req.key,
            "verb": req.verb, "ttl_secs": req.ttl_secs,
            "constraints": { "ip_hash": req.ip_hash, "byte_range_max": req.byte_range_max }
        });
        let out = serde_json::to_value(poi).unwrap_or(serde_json::json!({}));
        let resp = serde_json::json!({ "decision":"ASK", "poi": out });
        let _ = emit_audit_report("./tenants/_public", req.who.as_deref().unwrap_or("anonymous"), &meta, &resp, &serde_json::json!({"intent":"presign"})).await;
        return Ok(Json(PresignResp{ url: "".into(), meta: resp }));
    },
    PolicyDecision::ACK => { /* proceed to presign */ }
}

    // Choose provider
    let url = match req.backend.as_str() {
        "fs" => {
            #[cfg(feature="fs")]
            {
                let reg = engine_registry::fs_registry::FsRegistry::new("./.dev-registry");
                if req.verb == "GET" { reg.presign_get(&req.bucket, &req.key, req.ttl_secs).await }
                else { reg.presign_put(&req.bucket, &req.key, req.ttl_secs).await }
            }
            #[cfg(not(feature="fs"))]
            { Err(anyhow::anyhow!("fs backend not compiled")).map_err(|_| axum::http::StatusCode::BAD_REQUEST)? }
        },
        "s3" => {
            #[cfg(feature="s3")]
            {
                let reg = engine_registry::s3_registry::S3Registry::new_from_env().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
                if req.verb == "GET" { reg.presign_get(&req.bucket, &req.key, req.ttl_secs).await }
                else { reg.presign_put(&req.bucket, &req.key, req.ttl_secs).await }
            }
            #[cfg(not(feature="s3"))]
            { Err(anyhow::anyhow!("s3 backend not compiled")).map_err(|_| axum::http::StatusCode::BAD_REQUEST)? }
        },
        _ => { return Err(axum::http::StatusCode::BAD_REQUEST); }
    }.map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let meta = serde_json::json!({
        "backend": req.backend,
        "bucket": req.bucket,
        "key": req.key,
        "verb": req.verb,
        "ttl_secs": req.ttl_secs
    });


// Build resource descriptor
let res = serde_json::json!({
    "store": if req.backend=="s3" { "S3" } else { "FS" },
    "bucket": req.bucket,
    "prefix": "",
    "object": req.key,
    "verbs": [req.verb],
    "constraints": {
        "ip_hash": req.ip_hash,
        "byte_range_max": req.byte_range_max
    }
});

// Minimal signed grant (dev keys)
let audit_who = req.who.as_deref().unwrap_or("anonymous").to_string();
let grant = AccessGrant {
    kind: "access.grant.v1".into(),
    grant_id: ulid::Ulid::new().to_string(),
    sub: audit_who.clone(),
    resource: res,
    exp: (chrono::Utc::now() + chrono::Duration::seconds(req.ttl_secs as i64)).to_rfc3339(),
    iat: chrono::Utc::now().to_rfc3339(),
    nonce: format!("n-{}", ulid::Ulid::new()),
    seal: GrantSeal{ alg:"ed25519-blake3".into(), kid:"dev-2026-key-1".into(), sig:"".into() }
};
let sk_b64 = std::fs::read_to_string("secrets/dev/ed25519.private.b64").unwrap_or_default();
let grant = sign_grant(grant, &sk_b64, "dev-2026-key-1").map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
let grant_json = serde_json::to_value(&grant).unwrap();
    let paseto_token = paseto_sign(&grant, &sk_b64).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

// Emit audit span for grant issuance

    let who = req.who.as_deref().unwrap_or("anonymous");
    let _ = emit_audit_report("./tenants/_public", who, &meta, &serde_json::json!({"presigned_url": url, "grant": grant_json, "paseto": paseto_token}), &serde_json::json!({"intent":"presign"})).await;

    Ok(Json(PresignResp{ url, meta }))
}

use axum::{extract::Query, http::HeaderMap, response::IntoResponse};
use std::collections::HashMap;



#[derive(Serialize, Deserialize)]
struct RevocationManifest {
    kind: String,                 // "revocation.manifest.v1"
    updated_at: String,
    grants: Vec<String>,          // grant_ids
    sig: String                   // ed25519 over blake3(json without sig)
}

fn check_revocation_signed(grant_id: &str) -> bool {
    let p = std::path::Path::new("revoked_grants").join("manifest.json");
    if !p.exists() { return false; }
    let data = std::fs::read(&p).ok()?;
    let mf: RevocationManifest = serde_json::from_slice(&data).ok()?;
    if mf.kind != "revocation.manifest.v1" { return false; }

    // Verify signature
    let mut v: serde_json::Value = serde_json::from_slice(&data).ok()?;
    if let Some(obj) = v.as_object_mut() {
        obj.remove("sig");
    }
    let msg = serde_json::to_vec(&v).ok()?;
    let digest = blake3::hash(&msg);
    let pk_b64 = std::fs::read_to_string("secrets/dev/ed25519.public.b64").ok()?;
    let pk_bytes = base64::engine::general_purpose::STANDARD.decode(pk_b64.trim()).ok()?;
    let vk = ed25519_dalek::VerifyingKey::from_bytes(&pk_bytes.try_into().ok()?).ok()?;
    let sig_bytes = base64::engine::general_purpose::STANDARD.decode(mf.sig).ok()?;
    let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes.try_into().ok()?);
    if vk.verify(digest.as_bytes(), &sig).is_err() { return false; }

    mf.grants.iter().any(|g| g == grant_id)
}

fn check_revocation(grant_id: &str) -> bool {
    let path = std::path::Path::new("revoked_grants").join(format!("{}.json", grant_id));
    path.exists()
}

#[derive(Serialize)]
struct ProxyPoi {
    reason: String,
    violations: Vec<String>,
    hints: Vec<String>,
}

fn hash_ip(ip: &str) -> String {
    format!("iphash:{}", blake3::hash(ip.as_bytes()).to_hex())
}

async fn s3_proxy_handler(headers: HeaderMap, Query(q): Query<HashMap<String,String>>) -> Result<impl IntoResponse, axum::http::StatusCode> {
        if std::env::var("PROXY_DISABLE").ok().as_deref() == Some("1") { return Err(axum::http::StatusCode::SERVICE_UNAVAILABLE); }
    // Expect header X-LogLine-Grant: base64(JSON)
    let grant_b64 = headers.get("X-LogLine-Grant").ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    let grant_bytes = base64::engine::general_purpose::STANDARD.decode(grant_b64.as_bytes()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let grant: AccessGrant = serde_json::from_slice(&grant_bytes).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    // Verify signature
    let pk_b64 = std::fs::read_to_string("secrets/dev/ed25519.public.b64").unwrap_or_default();
    verify_grant(&grant, &pk_b64)?;

    // Basic expiry check
    let now = chrono::Utc::now();
    let exp = chrono::DateTime::parse_from_rfc3339(&grant.exp).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?.with_timezone(&chrono::Utc);
    if now > exp { return Err(axum::http::StatusCode::UNAUTHORIZED); }

    // Extract bucket/key from query (?bucket=...&key=...)
    let bucket = q.get("bucket").ok_or(axum::http::StatusCode::BAD_REQUEST)?;
    let key    = q.get("key").ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    // Policy check: object matches grant.resource.object
    let obj = grant.resource.get("object").and_then(|v| v.as_str()).ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    if obj != key { return Err(axum::http::StatusCode::UNAUTHORIZED); }

    // Backend: only s3 supported here
    #[cfg(feature="s3")]
    {
        let reg = engine_registry::s3_registry::S3Registry::new_from_env().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        let data = reg.get_bytes(bucket, key).await.map_err(|_| axum::http::StatusCode::BAD_GATEWAY)?;
        let meta = serde_json::json!({
            "proxy":"s3", "bucket":bucket, "key":key, "bytes": data.len()
        });
        let _ = emit_audit_report("./tenants/_public", "proxy", &serde_json::json!({"bucket":bucket,"key":key}), &meta, &serde_json::json!({"intent":"proxy_get"})).await;
        return Ok(([(axum::http::header::CONTENT_TYPE, "application/octet-stream")], data));
    }
    #[cfg(not(feature="s3"))]
    {
        Err(axum::http::StatusCode::NOT_IMPLEMENTED)
    }
}
