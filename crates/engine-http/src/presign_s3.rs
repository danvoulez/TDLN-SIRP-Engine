
#[cfg(feature = "s3")]
use aws_sdk_s3 as s3;
#[cfg(feature = "s3")]
use aws_sdk_s3::config::Region;
#[cfg(feature = "s3")]
use aws_sdk_s3::presigning::PresigningConfig;
use crate::presign::{Presigner, PresignIntent, PresignResponse};
use anyhow::Result;

pub struct S3Presigner {
    #[cfg(feature = "s3")]
    client: s3::Client,
    bucket_default: Option<String>,
}

impl S3Presigner {
    #[cfg(feature = "s3")]
    pub async fn from_env() -> Result<Self> {
        // Env: S3_ENDPOINT, S3_REGION, S3_ACCESS_KEY, S3_SECRET_KEY
        let endpoint = std::env::var("S3_ENDPOINT").ok();
        let region = std::env::var("S3_REGION").unwrap_or_else(|_| "auto".into());
        let mut loader = aws_config::from_env().region(Region::new(region));
        if let Some(ep) = endpoint {
            loader = loader.endpoint_url(ep);
        }
        let conf = loader.load().await;
        let client = s3::Client::new(&conf);
        Ok(Self{ client, bucket_default: std::env::var("S3_BUCKET_DEFAULT").ok() })
    }
}

#[async_trait::async_trait]
impl Presigner for S3Presigner {
    async fn presign(&self, intent: PresignIntent) -> Result<PresignResponse> {
        #[cfg(not(feature="s3"))]
        {
            anyhow::bail!("s3 feature not enabled");
        }
        #[cfg(feature="s3")]
        {
            let bucket = if intent.resource.bucket.is_empty() {
                self.bucket_default.clone().unwrap_or_default()
            } else { intent.resource.bucket.clone() };

            let key = format!("{}/{}", intent.resource.prefix.trim_matches('/'), intent.resource.object);

            let exp = std::time::Duration::from_secs(intent.ttl_seconds);
            let when = chrono::Utc::now() + chrono::Duration::seconds(intent.ttl_seconds as i64);
            let presigned_url = match intent.resource.verb.as_str() {
                "PUT" => {
                    let req = self.client.put_object().bucket(&bucket).key(&key).presigned(PresigningConfig::expires_in(exp)?).await?;
                    req.uri().to_string()
                }
                _ => {
                    let req = self.client.get_object().bucket(&bucket).key(&key).presigned(PresigningConfig::expires_in(exp)?).await?;
                    req.uri().to_string()
                }
            };
            Ok(PresignResponse{
                url: presigned_url,
                expires_at: when.to_rfc3339(),
                grant: serde_json::json!({ "kind":"access.grant.v1", "mock": false, "bucket": bucket, "key": key })
            })
        }
    }
}
