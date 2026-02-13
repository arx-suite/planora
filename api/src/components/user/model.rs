use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Deactivated,
    Banned,
}

impl UserStatus {
    pub fn is_pending(&self) -> bool {
        matches!(self, UserStatus::Pending)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, UserStatus::Active)
    }

    pub fn is_suspended(&self) -> bool {
        matches!(self, UserStatus::Suspended)
    }

    pub fn is_deactivated(&self) -> bool {
        matches!(self, UserStatus::Deactivated)
    }

    pub fn is_banned(&self) -> bool {
        matches!(self, UserStatus::Banned)
    }
}

impl From<UserStatus> for sea_query::Value {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Pending => "pending".into(),
            UserStatus::Active => "active".into(),
            UserStatus::Suspended => "suspended".into(),
            UserStatus::Deactivated => "deactivated".into(),
            UserStatus::Banned => "banned".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserNotificationSettings {
    pub email: Option<bool>,
    pub push: Option<bool>,
    pub inapp: Option<bool>,
    pub digest: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRow {
    pub user_id: Uuid,

    // user status
    pub status: UserStatus,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub locked_until: Option<DateTime<Utc>>,
    pub email_verified_at: Option<DateTime<Utc>>,

    // core identity
    pub usertag: String,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,

    // security
    pub password_hash: Option<String>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub password_reset_required: bool,

    // preferences
    pub preferences: Json<UserPreferences>,
    pub notifications_settings: Json<UserNotificationSettings>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// helper functions for actix-web extensions
impl UserRow {
    #[inline]
    pub fn add(self, req: &actix_web::dev::ServiceRequest) {
        <actix_web::dev::ServiceRequest as actix_web::HttpMessage>::extensions_mut(req)
            .insert(self);
    }

    #[inline]
    pub fn extract(req: &actix_web::HttpRequest) -> Result<UserRow, crate::common::ApiError> {
        <actix_web::HttpRequest as actix_web::HttpMessage>::extensions(req)
            .get::<UserRow>()
            .cloned()
            .ok_or_else(|| crate::common::ApiError::Unauthorized("User not authenticated".into()))
    }
}

#[derive(sea_query::Iden)]
pub enum Users {
    Table,
    UserId,
    Status,
    DeactivatedAt,
    LockedUntil,
    EmailVerifiedAt,
    Usertag,
    Username,
    Email,
    AvatarUrl,
    PasswordHash,
    PasswordChangedAt,
    PasswordResetRequired,
    Preferences,
    NotificationSettings,
    CreatedAt,
    UpdatedAt,
}

// user identities
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderData {}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserIdentityRow {
    pub user_id: Uuid,
    pub provider: String,
    pub provider_email: Option<String>,
    pub data: Option<Json<ProviderData>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sea_query::Iden)]
pub enum UserIdentities {
    Table,
    UserId,
    Provider,
    ProviderEmail,
    Data,
    CreatedAt,
    UpdatedAt,
}
