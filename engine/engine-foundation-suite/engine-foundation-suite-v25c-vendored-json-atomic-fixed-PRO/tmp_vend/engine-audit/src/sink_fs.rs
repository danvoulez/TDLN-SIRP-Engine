
use anyhow::Result;
use std::path::Path;
use crate::report::AuditReportV1;

pub struct FsAudit {
  pub dir: String
}
impl FsAudit {
  pub fn new<P: AsRef<Path>>(dir:P)->Self { Self{ dir: dir.as_ref().to_string_lossy().into() } }
  pub fn emit(&self, a:&AuditReportV1) -> Result<()> {
      std::fs::create_dir_all(&self.dir)?;
      let path = format!("{}/{}_{}.json", self.dir, a.kind.replace(".","-"), a.audit_id);
      std::fs::write(path, serde_json::to_string_pretty(a)?)?;
      Ok(())
  }
}
