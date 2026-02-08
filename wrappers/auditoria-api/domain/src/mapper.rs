use crate::schema::InsuranceClaim;
use anyhow::Result;
use serde_json::json;

/// Domain -> Engine manifest
pub fn to_engine_manifest(c: &InsuranceClaim) -> Result<serde_json::Value> {
    Ok(json!({
        "policy": "policy://default",
        "data": { "claim_id": c.claim_id, "amount": c.amount, "incident_date": c.incident_date, "evidence_cids": c.evidence_cids }
    }))
}
