
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub name: String,
    pub version: String,
    pub cid: String,
    pub meta: serde_json::Value,
}

#[async_trait::async_trait]
pub trait RegistryProvider: Send + Sync {
    async fn put(&self, entry: RegistryEntry) -> Result<()>;
    async fn get(&self, name: &str, version: &str) -> Result<Option<RegistryEntry>>;
}

/// FileSystem registry (JSON lines under a root dir)
pub struct FsRegistry {
    root: std::path::PathBuf
}
impl FsRegistry {
    pub fn new<P: Into<std::path::PathBuf>>(root:P) -> Self { Self { root: root.into() } }
    fn path_for(&self, name:&str, version:&str)->std::path::PathBuf{
        self.root.join(format!("{name}/{version}.json"))
    }
}
#[async_trait::async_trait]
impl RegistryProvider for FsRegistry {
    async fn put(&self, entry: RegistryEntry) -> Result<()> {
        let p = self.path_for(&entry.name, &entry.version);
        if let Some(dir) = p.parent() { tokio::fs::create_dir_all(dir).await?; }
        let data = serde_json::to_vec_pretty(&entry)?;
        tokio::fs::write(p, data).await?;
        Ok(())
    }
    async fn get(&self, name:&str, version:&str) -> Result<Option<RegistryEntry>> {
        let p = self.path_for(name, version);
        if !p.exists() { return Ok(None); }
        let data = tokio::fs::read(p).await?;
        let e: RegistryEntry = serde_json::from_slice(&data)?;
        Ok(Some(e))
    }
}

/// S3-compatible placeholder (R2/MinIO). Concrete client wired in extras crate.
pub struct S3Registry {
    pub bucket: String,
    pub prefix: String,
}
#[async_trait::async_trait]
impl RegistryProvider for S3Registry {
    async fn put(&self, _entry: RegistryEntry) -> Result<()> {
        anyhow::bail!("S3Registry is provided by engine-extras with real S3 client feature");
    }
    async fn get(&self, _name:&str, _version:&str) -> Result<Option<RegistryEntry>> {
        anyhow::bail!("S3Registry is provided by engine-extras with real S3 client feature");
    }
}
