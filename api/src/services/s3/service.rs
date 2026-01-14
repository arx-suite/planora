use actix_multipart::form::tempfile::TempFile;
use aws_sdk_s3 as s3;
use aws_sdk_s3::presigning::PresigningConfig;
use s3::config::{Builder as S3ConfigBuilder, Credentials};
use s3::primitives::ByteStream;
use std::{path::Path, time::Duration};
use uuid::Uuid;

use super::asset::SignedUrlProvider;
use super::{S3Error, S3Result, S3ServiceI};
use crate::common::utils;

const ENV_AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
const ENV_AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
const ENV_AWS_S3_ENDPOINT: &str = "AWS_S3_ENDPOINT";

#[derive(Debug, Clone)]
pub struct S3Service {
    client: s3::Client,
    endpoint: String,
}

impl S3Service {
    async fn from_env(app_name: String) -> S3Result<Self> {
        let name = aws_config::AppName::new(app_name).expect("app name should be valid");

        let access_key_id = utils::get_env::<String>(ENV_AWS_ACCESS_KEY_ID).unwrap();
        let secret_access_key = utils::get_env::<String>(ENV_AWS_SECRET_ACCESS_KEY).unwrap();
        let s3_endpoint = utils::get_env::<url::Url>(ENV_AWS_S3_ENDPOINT).unwrap();

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
            endpoint: s3_endpoint.to_string(),
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

#[async_trait::async_trait]
impl SignedUrlProvider for S3Service {
    async fn sign_get(&self, bucket: &str, key: &str, expires: Duration) -> S3Result<String> {
        let presigned = self
            .client()
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(
                PresigningConfig::expires_in(expires)
                    .map_err(|e| S3Error::Config(e.to_string()))?,
            )
            .await?;

        Ok(presigned.uri().to_string())
    }
}

#[tracing::instrument(
    name = "service.s3",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub async fn init(app_name: String) -> S3Service {
    let s3_service = S3Service::from_env(app_name)
        .await
        .expect("Failed to setup S3");

    s3_service
}
