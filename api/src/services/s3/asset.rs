use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;

use super::{S3Result, S3ServiceI};

#[async_trait::async_trait]
pub trait SignedUrlProvider {
    async fn sign_get(&self, bucket: &str, key: &str, expires: Duration) -> S3Result<String>;
}

#[derive(Debug, Clone)]
pub struct AssetRef {
    pub bucket: String,
    pub key: String,
    pub visibility: Visibility,
}

pub struct AssetResolver<S> {
    s3: S,
    public_base: String,
}

impl<S> AssetResolver<S> {
    pub fn new(s3: S, public_base: String) -> Self {
        Self { s3, public_base }
    }
}

impl<S> AssetResolver<S>
where
    S: S3ServiceI + SignedUrlProvider,
{
    pub async fn resolve(&self, asset: &AssetRef) -> S3Result<String> {
        match asset.visibility {
            Visibility::Public => Ok(format!(
                "{}/{}/{}",
                self.public_base, asset.bucket, asset.key
            )),

            Visibility::Private => {
                self.s3
                    .sign_get(&asset.bucket, &asset.key, Duration::from_secs(300))
                    .await
            }
        }
    }
}

impl AssetRef {
    pub fn new(bucket: String, key: String, visibility: Visibility) -> Self {
        Self {
            bucket,
            key,
            visibility,
        }
    }

    pub fn private(bucket: String, key: String) -> Self {
        Self {
            bucket,
            key,
            visibility: Visibility::Private,
        }
    }

    pub fn public(bucket: String, key: String) -> Self {
        Self {
            bucket,
            key,
            visibility: Visibility::Public,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    Public,
    Private,
}
