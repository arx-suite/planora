use std::sync::Arc;
use tokio::sync::RwLock;

use aws_config::AppName;
use aws_sdk_s3 as s3;
use s3::config::{Builder as S3ConfigBuilder, Credentials};

const APP_NAME: &'static str = "arx-gatehouse";

#[derive(Debug, Clone)]
pub struct S3Service {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Debug, Clone)]
struct Inner {
    client: s3::Client,
    bucket: String,
}

impl Inner {
    fn new(client: s3::Client, bucket: impl ToString) -> Self {
        Self {
            client,
            bucket: bucket.to_string(),
        }
    }
}

impl S3Service {
    pub async fn from_env() -> Self {
        let app_name = AppName::new(APP_NAME).expect("app name should be valid");

        let access_key_id =
            std::env::var("AWS_ACCESS_KEY_ID").expect("`AWS_ACCESS_KEY_ID` must be set");
        let secret_access_key =
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("`AWS_SECRET_ACCESS_KEY` must be set");
        let s3_endpoint = std::env::var("AWS_S3_ENDPOINT").expect("`AWS_S3_ENDPOINT` must be set");

        let creds = Credentials::new(access_key_id, secret_access_key, None, None, APP_NAME);

        let base_config = aws_config::from_env()
            .credentials_provider(creds)
            .load()
            .await
            .to_builder()
            .app_name(app_name)
            .build();

        let s3_config = S3ConfigBuilder::from(&base_config)
            .endpoint_url(s3_endpoint)
            .force_path_style(true)
            .build();

        let client = s3::Client::from_conf(s3_config);

        Self {
            inner: Arc::new(RwLock::new(Inner::new(client, APP_NAME))),
        }
    }
}
