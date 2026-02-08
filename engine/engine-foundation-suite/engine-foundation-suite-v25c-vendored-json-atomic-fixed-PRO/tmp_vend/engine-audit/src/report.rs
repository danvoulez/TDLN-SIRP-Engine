
use serde::{Serialize, Deserialize};
use engine_core::model::ExecutionReceipt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReportV1 {
  pub kind: String, // "audit.report.v1"
  pub audit_id: String,
  pub ts: String,
  pub actor: String,
  pub plan: serde_json::Value,
  pub limits: serde_json::Value,
  pub proofs: serde_json::Value,
  pub receipt: ExecutionReceipt,
}
