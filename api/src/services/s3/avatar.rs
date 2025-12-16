use std::path::Path;

use actix_multipart::form::tempfile::TempFile;
use actix_web::mime::{IMAGE_JPEG, IMAGE_PNG, Mime};
use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;

use super::{S3Error, S3Result, S3ServiceI};

#[async_trait::async_trait]
pub trait AvatarStorage {
    const BUCKET_NAME: &'static str = "avatars";
    const MAX_FILE_SIZE: usize = 2 * 1024 * 1024;
    const SUPPORTED_FILE_FORMATS: [Mime; 2] = [IMAGE_JPEG, IMAGE_PNG];
    const SUPPORTED_FILE_EXTENSIONS: [&str; 4] = ["png", "webp", "jpg", "jpeg"];

    async fn upload_avatar(&self, user_id: Uuid, file: TempFile) -> S3Result<String>;
    async fn remove_avatar(&self, user_id: Uuid) -> S3Result<()>;
    fn key_avatar(user_id: Uuid, ext: &str) -> String;
}

#[async_trait::async_trait]
impl<I> AvatarStorage for I
where
    I: S3ServiceI,
{
    async fn upload_avatar(&self, user_id: Uuid, file: TempFile) -> S3Result<String> {
        let ext = extract_extension(&file)?;
        let content_type = extract_content_type(&file)?;

        if !Self::SUPPORTED_FILE_EXTENSIONS.contains(&ext.as_ref()) {
            return Err(S3Error::UnsupportedExtension);
        }

        if !Self::SUPPORTED_FILE_FORMATS.contains(&content_type) {
            return Err(S3Error::UnsupportedContent);
        }

        let key = Self::key_avatar(user_id, &ext);
        let body = ByteStream::from_path(&file.file)
            .await
            .map_err(|e| S3Error::ByteStream(e))?;

        self.put_object(Self::BUCKET_NAME, &key, content_type.as_ref(), body)
            .await?;

        Ok(self.object_url(Self::BUCKET_NAME, &key))
    }

    async fn remove_avatar(&self, user_id: Uuid) -> S3Result<()> {
        // TODO: change the hardcoded extension
        let key = Self::key_avatar(user_id, "png");
        self.delete_object(Self::BUCKET_NAME, &key).await?;
        Ok(())
    }

    fn key_avatar(user_id: Uuid, ext: &str) -> String {
        format!("{}/profile.{}", user_id, ext)
    }
}

fn extract_extension(file: &TempFile) -> S3Result<String> {
    let ext = file
        .file_name
        .as_deref()
        .and_then(|f| Path::new(f).extension())
        .and_then(|e| e.to_str())
        .ok_or_else(|| S3Error::InvalidData("missing file extension".into()))?;

    Ok(ext.to_lowercase())
}

fn extract_content_type(file: &TempFile) -> S3Result<&Mime> {
    let ct = file
        .content_type
        .as_ref()
        .ok_or_else(|| S3Error::InvalidData("missing content type".into()))?;

    Ok(ct)
}
