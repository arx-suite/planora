use actix_multipart::form::tempfile::TempFile;
use aws_sdk_s3 as s3;
use s3::config::{Builder as S3ConfigBuilder, Credentials};
use s3::primitives::ByteStream;
use std::path::Path;
use uuid::Uuid;

use super::{S3Error, S3Result, S3ServiceI};

#[derive(Debug, Clone)]
pub struct S3Service {
    client: s3::Client,
    endpoint: String,
}

impl S3Service {
    pub async fn from_env(app_name: String) -> S3Result<Self> {
        let name = aws_config::AppName::new(app_name).expect("app name should be valid");

        let access_key_id =
            std::env::var("AWS_ACCESS_KEY_ID").expect("`AWS_ACCESS_KEY_ID` must be set");
        let secret_access_key =
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("`AWS_SECRET_ACCESS_KEY` must be set");
        let s3_endpoint = std::env::var("AWS_S3_ENDPOINT").expect("`AWS_S3_ENDPOINT` must be set");

        let creds = Credentials::new(access_key_id, secret_access_key, None, None, "minio");

        let base_config = aws_config::from_env()
            .credentials_provider(creds)
            .load()
            .await
            .to_builder()
            .app_name(name)
            .build();

        let s3_config = S3ConfigBuilder::from(&base_config)
            .endpoint_url(s3_endpoint.clone())
            .force_path_style(true)
            .build();

        let client = s3::Client::from_conf(s3_config);

        Ok(Self {
            client,
            endpoint: s3_endpoint,
        })
    }
}

#[async_trait::async_trait]
impl S3ServiceI for S3Service {
    fn client(&self) -> &aws_sdk_s3::Client {
        &self.client
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        body: ByteStream,
    ) -> S3Result<()> {
        self.client()
            .put_object()
            .bucket(bucket)
            .key(key)
            .content_type(content_type)
            .acl(s3::types::ObjectCannedAcl::Private)
            .body(body)
            .send()
            .await?;

        Ok(())
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> S3Result<()> {
        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }
}
