
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub deterministic: bool,
    pub fuel: u64,
    pub memory_max_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EerWasm {
    pub runtime: RuntimeMeta,
    pub config: RuntimeConfig,
    pub digests: Digests,
    pub wasmtime: WasmtimeMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMeta { pub name: String, pub version: String, pub hash: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmtimeMeta { pub version: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Digests { pub unit_cid: String, pub policy_cid: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptSeal { pub alg: String, pub kid: String, pub sig: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStep { pub kind: String, pub cid: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptProof { pub seal: ReceiptSeal, pub hash_chain: Vec<ChainStep>, pub eer: Option<serde_json::Value> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub kind: String,
    pub realm: String,
    pub decision: String,
    pub output_cid: String,
    pub proof: ReceiptProof,
    #[serde(default)] pub refs: Vec<serde_json::Value>,
    pub links: Links,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links { pub card_url: String }

pub trait CertifiedRuntime {
    fn execute(&self, unit_bytes: &[u8], input_json: &serde_json::Value, cfg: &RuntimeConfig) -> anyhow::Result<Card>;
}
