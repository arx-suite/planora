pub mod auth;
pub mod cache;
pub mod db;
pub mod mail;
pub mod s3;

pub use auth::AuthService;
pub use cache::CacheService;
pub use db::DbService;
pub use mail::MailService;
pub use s3::{AvatarStorage, S3Service};

#[derive(Debug, Clone)]
pub struct AppService {
    pub auth: auth::AuthService,
    pub db: db::DbService,
    pub cache: cache::CacheService,
    pub s3: s3::S3Service,
    pub mail: mail::MailService,
}

#[tracing::instrument(
    name = "services.initialize",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub async fn init(app_name: String) -> AppService {
    AppService {
        auth: auth::service::init(),
        db: db::service::init().await,
        cache: cache::service::init(),
        s3: s3::service::init(app_name.clone()).await,
        mail: mail::service::init(),
    }
}
