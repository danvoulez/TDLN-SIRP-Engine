
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignIntent {
    pub actor: String,
    pub resource: PresignResource,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignResource {
    pub store: String,        // generic tag, e.g., "S3Compatible"
    pub bucket: String,
    pub prefix: String,
    pub object: String,
    pub verb: String,         // "GET" or "PUT"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignResponse {
    pub url: String,
    pub expires_at: String,
    pub grant: serde_json::Value, // serialized AccessGrant for the caller to persist
}

/// Abstraction: wrappers implement a concrete presigner over any backend (S3/R2/MinIO/etc).
#[async_trait::async_trait]
pub trait Presigner: Send + Sync + 'static {
    async fn presign(&self, intent: PresignIntent) -> anyhow::Result<PresignResponse>;
}

/// A no-op presigner useful for tests; returns a synthetic URL.
pub struct StubPresigner;
#[async_trait::async_trait]
impl Presigner for StubPresigner {
    async fn presign(&self, intent: PresignIntent) -> anyhow::Result<PresignResponse> {
        let exp = chrono::Utc::now() + chrono::Duration::seconds(intent.ttl_seconds as i64);
        Ok(PresignResponse {
            url: format!("stub://{}/{}/{}?verb={}&exp={}", intent.resource.bucket, intent.resource.prefix.trim_matches('/'), intent.resource.object, intent.resource.verb, intent.ttl_seconds),
            expires_at: exp.to_rfc3339(),
            grant: serde_json::json!({ "kind":"access.grant.v1", "mock": true })
        })
    }
}
