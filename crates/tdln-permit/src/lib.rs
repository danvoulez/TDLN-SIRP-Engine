use anyhow::{anyhow, Result};
use base64::Engine as _;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRef {
    pub decision: String,
    pub engine: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermitInner {
    pub version: String,
    pub rid: String,
    pub action: String,
    pub args: Value,
    pub target: Value,
    pub who: String,
    pub exp: String,
    pub policy: PolicyRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReady {
    pub request_id: String,
    pub ts: String,
    pub by: String,
    pub permit: PermitInner,
    pub permit_sig: String,
}

fn sort_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();

            let mut sorted = serde_json::Map::with_capacity(map.len());
            for k in keys {
                let mut v = map.remove(&k).unwrap_or_else(|| Value::Null);
                sort_value(&mut v);
                sorted.insert(k, v);
            }

            *map = sorted;
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                sort_value(v);
            }
        }
        _ => {}
    }
}

pub fn canonicalize_json(value: &Value) -> Result<Vec<u8>> {
    let mut v = value.clone();
    sort_value(&mut v);
    Ok(serde_json::to_vec(&v)?)
}

pub fn blake3_hex(bytes: &[u8]) -> String {
    let h = blake3::hash(bytes);
    format!("b3:{}", h.to_hex())
}

pub fn verify_permit(ready: &ExecutionReady, input_canon: &[u8], tdln_pubkey: &[u8]) -> Result<()> {
    if ready.permit.policy.decision.as_str() != "ALLOW" {
        return Err(anyhow!("decision is not ALLOW"));
    }
    let exp = OffsetDateTime::parse(
        &ready.permit.exp,
        &time::format_description::well_known::Rfc3339,
    )?;
    if OffsetDateTime::now_utc() > exp {
        return Err(anyhow!("permit expired"));
    }
    let tdln_pubkey: &[u8; 32] = tdln_pubkey
        .try_into()
        .map_err(|_| anyhow!("tdln_pubkey must be 32 bytes"))?;
    let vk = VerifyingKey::from_bytes(tdln_pubkey)?;
    let permit_val = serde_json::to_value(&ready.permit)?;
    let permit_bytes = canonicalize_json(&permit_val)?;
    let sig_bytes = base64::engine::general_purpose::STANDARD.decode(&ready.permit_sig)?;
    let sig = Signature::from_slice(&sig_bytes)?;
    vk.verify(&permit_bytes, &sig)?;

    let expect_hash = blake3_hex(input_canon);
    if expect_hash != ready.permit.policy.hash {
        return Err(anyhow!("input hash mismatch"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_hash_is_stable() {
        let v: Value = serde_json::json!({
            "b": 2,
            "a": { "y": 2, "x": 1 }
        });
        let h1 = blake3_hex(&canonicalize_json(&v).unwrap());
        let v2: Value = serde_json::json!({
            "a": { "x": 1, "y": 2 },
            "b": 2
        });
        let h2 = blake3_hex(&canonicalize_json(&v2).unwrap());
        assert_eq!(h1, h2);
    }
}
