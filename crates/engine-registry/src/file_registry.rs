
use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::schema::EngineRegistryEntry;

pub struct FileRegistry { pub dir: PathBuf }
impl FileRegistry {
    pub fn new<P: AsRef<Path>>(dir:P)->Self { Self{ dir: dir.as_ref().into() } }
    pub fn put(&self, entry:&EngineRegistryEntry) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.dir)?;
        let path = self.dir.join(format!("{}_{}.json", entry.name, entry.version));
        std::fs::write(&path, serde_json::to_string_pretty(entry)?)?;
        Ok(path)
    }
    pub fn get(&self, name:&str, version:&str) -> Result<Option<EngineRegistryEntry>> {
        let path = self.dir.join(format!("{}_{}.json", name, version));
        if path.exists() {
            let s = std::fs::read_to_string(&path)?;
            Ok(Some(serde_json::from_str(&s)?))
        } else {
            Ok(None)
        }
    }
}
