
#[cfg(feature = "s3")]
pub mod s3_compatible {
    use anyhow::{Result, anyhow};
    use engine_core::providers::ReceiptSink;
    use engine_core::model::ExecutionReceipt;

    pub struct S3CompatibleSink {
        pub bucket: String,
        pub prefix: String,
        pub endpoint: String,
        pub region: String,
    }
    impl S3CompatibleSink {
        pub fn new(bucket:&str, prefix:&str, endpoint:&str, region:&str)->Self {
            Self{ bucket: bucket.to_string(), prefix: prefix.to_string(), endpoint: endpoint.to_string(), region: region.to_string() }
        }
        fn key_for(&self, r:&ExecutionReceipt) -> String {
            let day = r.timestamp.split('T').first().unwrap_or("YYYY-MM-DD");
            format!("{}/receipt/{}/{}_{}.json", self.prefix.trim_end_matches('/'), day.replace('-','/'), r.chip_id, r.timestamp.replace(':','-'))
        }
    }

    #[async_trait::async_trait]
    impl ReceiptSink for S3CompatibleSink {
        fn emit(&self, r:&ExecutionReceipt) -> Result<()> {
            let body = serde_json::to_vec(r)?;
            let key = self.key_for(r);
            Err(anyhow!(format!("Enable feature 's3' and wire an async uploader to put {} bytes to s3://{}/{}", body.len(), self.bucket, key)))
        }
    }
}
