use actix_web::{HttpRequest, Responder, get};

use super::model::{UserPreferences, UserRow, UserStatus};
use crate::common::{ApiError, ApiResult};

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    user_id: uuid::Uuid,
    status: UserStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_verified_at: Option<chrono::DateTime<chrono::Utc>>,
    user_tag: String,
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    preferences: UserPreferences,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for UserProfile {
    fn from(value: UserRow) -> Self {
        Self {
            user_id: value.user_id,
            status: value.status,
            email_verified_at: value.email_verified_at,
            user_tag: value.usertag,
            username: value.username,
            email: value.email,
            avatar_url: value.avatar_url,
            preferences: value.preferences.0,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[utoipa::path(
    get,
    path = "/profile",
    tag = "Profile",
    responses(
        (status = 200, description = "Profile data", body = UserProfile),
        // TODO: common response handled by Auth middleware
        // (status = 403, description = "Account has been deactivated"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "profile.get",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty
    )
)]
#[get("")]
async fn get_profile(req: HttpRequest) -> Result<impl Responder, ApiError> {
    let user = UserRow::extract_extension(&req)?;
    tracing::Span::current().record("user_id", &user.user_id.to_string());

    ApiResult::to_ok_response("Profile data", UserProfile::from(user))
}
