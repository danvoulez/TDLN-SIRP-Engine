
use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMeta {
    pub bucket: String,
    pub key: String,
    pub size: Option<u64>,
    pub etag: Option<String>,
    pub cid_b3: Option<String>,
}

#[async_trait]
pub trait RegistryProvider: Send + Sync {
    async fn put_bytes(&self, bucket: &str, key: &str, bytes: &[u8]) -> Result<ObjectMeta>;
    async fn get_bytes(&self, bucket: &str, key: &str) -> Result<Vec<u8>>;
    async fn head(&self, bucket: &str, key: &str) -> Result<ObjectMeta>;
    async fn presign_get(&self, bucket: &str, key: &str, ttl_secs: u64) -> Result<String>;
    async fn presign_put(&self, bucket: &str, key: &str, ttl_secs: u64) -> Result<String>;
}

#[cfg(feature="fs")]
pub mod fs_registry {
    use super::*;
    use std::path::PathBuf;
    use tokio::fs;
    use blake3::Hasher;

    #[derive(Clone)]
    pub struct FsRegistry { pub root: PathBuf }
    impl FsRegistry { pub fn new(root: impl Into<PathBuf>) -> Self { Self{ root: root.into() } } }

    #[async_trait]
    impl RegistryProvider for FsRegistry {
        async fn put_bytes(&self, bucket:&str, key:&str, bytes:&[u8]) -> Result<ObjectMeta> {
            let p = self.root.join(bucket).join(key);
            if let Some(dir) = p.parent() { fs::create_dir_all(dir).await?; }
            fs::write(&p, bytes).await?;
            let mut h = Hasher::new(); h.update(bytes);
            Ok(ObjectMeta{
                bucket: bucket.to_string(),
                key: key.to_string(),
                size: Some(bytes.len() as u64),
                etag: None,
                cid_b3: Some(format!("b3:{}", h.finalize().to_hex())),
            })
        }
        async fn get_bytes(&self, bucket:&str, key:&str) -> Result<Vec<u8>> {
            let p = self.root.join(bucket).join(key);
            Ok(fs::read(p).await?)
        }
        async fn head(&self, bucket:&str, key:&str) -> Result<ObjectMeta> {
            let p = self.root.join(bucket).join(key);
            let md = fs::metadata(&p).await?;
            Ok(ObjectMeta{ bucket: bucket.to_string(), key: key.to_string(), size: Some(md.len()), etag: None, cid_b3: None })
        }
        async fn presign_get(&self, bucket:&str, key:&str, _ttl_secs:u64) -> Result<String> {
            Ok(format!("fs://{}/{}", bucket, key))
        }
        async fn presign_put(&self, bucket:&str, key:&str, _ttl_secs:u64) -> Result<String> {
            Ok(format!("fs://{}/{}", bucket, key))
        }
    }
}

#[cfg(feature="s3")]
pub mod s3_registry {
    use super::*;
    use aws_sdk_s3::{Client, config::Region, primitives::ByteStream};
    use aws_sdk_s3::types::SdkError;
    use blake3::Hasher;

    #[derive(Clone)]
    pub struct S3Registry { pub client: Client }

    impl S3Registry {
        pub async fn new_from_env() -> Result<Self> {
            // Works for Cloudflare R2 and MinIO (provide endpoint via env AWS_ENDPOINT_URL / AWS_REGION)
            let mut loader = aws_config::from_env();
            let conf = loader.load().await;
            let client = Client::new(&conf);
            Ok(Self{ client })
        }
    }

    #[async_trait]
    impl RegistryProvider for S3Registry {
        async fn put_bytes(&self, bucket:&str, key:&str, bytes:&[u8]) -> Result<ObjectMeta> {
            self.client.put_object()
                .bucket(bucket).key(key)
                .body(ByteStream::from(bytes.to_vec()))
                .send().await?;
            let mut h = Hasher::new(); h.update(bytes);
            Ok(ObjectMeta{
                bucket: bucket.to_string(), key: key.to_string(),
                size: Some(bytes.len() as u64), etag: None,
                cid_b3: Some(format!("b3:{}", h.finalize().to_hex()))
            })
        }
        async fn get_bytes(&self, bucket:&str, key:&str) -> Result<Vec<u8>> {
            let out = self.client.get_object().bucket(bucket).key(key).send().await?;
            let data = out.body.collect().await?.into_bytes().to_vec();
            Ok(data)
        }
        async fn head(&self, bucket:&str, key:&str) -> Result<ObjectMeta> {
            let out = self.client.head_object().bucket(bucket).key(key).send().await?;
            Ok(ObjectMeta{
                bucket: bucket.to_string(), key: key.to_string(),
                size: out.content_length().map(|v| v as u64), etag: out.e_tag().map(|s| s.to_string()), cid_b3: None
            })
        }
        async fn presign_get(&self, bucket:&str, key:&str, _ttl_secs:u64) -> Result<String> {
            // Simplified: in real impl, use PresigningConfig. For R2/MinIO this will require custom presigners.
            Ok(format!("s3://{}/{}", bucket, key))
        }
        async fn presign_put(&self, bucket:&str, key:&str, _ttl_secs:u64) -> Result<String> {
            Ok(format!("s3://{}/{}", bucket, key))
        }
    }
}
