
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrant {
    pub kind: String,                 // "access.grant.v1"
    pub grant_id: String,             // ULID
    pub sub: String,                  // subject (email, id, etc.)
    pub tenants: Vec<String>,         // scopes
    pub resource: GrantResource,
    pub exp: String,                  // RFC3339
    pub iat: String,                  // RFC3339
    pub nonce: String,
    pub seal: GrantSeal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantResource {
    pub store: String,                // e.g., "S3Compatible"
    pub bucket: String,
    pub prefix: String,
    pub object: Option<String>,
    pub verbs: Vec<String>,           // ["GET"], ["PUT"]
    pub constraints: Option<GrantConstraints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConstraints {
    pub ip_hash: Option<String>,
    pub byte_range_max: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantSeal {
    pub alg: String,                  // "ed25519-blake3"
    pub kid: String,
    pub sig: String,                  // base64
}
