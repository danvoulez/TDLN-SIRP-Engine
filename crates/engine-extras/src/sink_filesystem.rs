use anyhow::Result;
use engine_core::model::ExecutionReceipt;
use engine_core::providers::ReceiptSink;
use std::fs;
use std::path::Path;

pub struct FsSink {
    pub dir: String,
}
impl FsSink {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            dir: dir.as_ref().to_string_lossy().into(),
        }
    }
}
impl ReceiptSink for FsSink {
    fn emit(&self, receipt: &ExecutionReceipt) -> Result<()> {
        fs::create_dir_all(&self.dir)?;
        let path = format!(
            "{}/{}_{}.json",
            self.dir,
            receipt.chip_id,
            receipt.timestamp.replace(":", "-")
        );
        std::fs::write(path, serde_json::to_string_pretty(receipt)?)?;
        Ok(())
    }
}
