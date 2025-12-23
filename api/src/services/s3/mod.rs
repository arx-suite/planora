#![allow(dead_code, unused_imports)]

mod asset;
mod avatar;
mod error;
pub mod service;

pub use avatar::AvatarStorage;
pub use error::{S3Error, S3Result};
pub use service::S3Service;

#[async_trait::async_trait]
pub trait S3ServiceI: Send + Sync {
    fn client(&self) -> &aws_sdk_s3::Client;
    fn endpoint(&self) -> &str;

    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        body: aws_sdk_s3::primitives::ByteStream,
    ) -> S3Result<()>;

    async fn delete_object(&self, bucket: &str, key: &str) -> S3Result<()>;

    fn object_url(&self, bucket: &str, key: &str) -> String {
        format!("{}/{}/{}", self.endpoint(), bucket, key)
    }
}
