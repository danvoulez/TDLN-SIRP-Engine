
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Seal {
    pub alg: String,      // "ed25519-blake3"
    pub kid: String,      // key id
    pub sig: String,      // base64 signature over canonical payload
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChainStep {
    pub kind: String,     // "input"|"exec"|"output"
    pub cid: String,      // "cid:b3:<hex>"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proof {
    pub seal: Seal,
    pub hash_chain: Vec<ChainStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefItem {
    pub kind: String,
    pub cid: String,              // "cid:b3:<hex>"
    pub media_type: String,
    #[serde(default)]
    pub size: Option<u64>,
    pub hrefs: Vec<String>,
    #[serde(default)]
    pub private: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Links {
    
    #[serde(default)]
    pub url: String,
#[serde(skip_serializing_if="String::is_empty", default)]
    pub card_url: String,         // "https://cert.tdln.foundry/r/b3:<run_cid>"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub kind: String,             // "receipt.card.v1"
    pub realm: String,            // "trust"
    pub decision: String,         // "ACK"|"ASK"|"NACK"|"RUNNING" (transient)
    pub unit_id: Option<String>,  // "cid:b3:<hex>"
    pub policy_id: Option<String>,
    pub output_cid: String,       // "cid:b3:<hex>"
    pub proof: Proof,
    #[serde(default)]
    pub poi: Option<serde_json::Value>,
    #[serde(default)]
    pub refs: Vec<RefItem>,
    pub links: Links,
}
