use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserRow;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SigninPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    user_id: Uuid,
    user_tag: Option<String>,
    username: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
}

impl From<UserRow> for UserProfile {
    fn from(value: UserRow) -> Self {
        Self {
            user_id: value.user_id,
            user_tag: value.user_tag,
            username: value.username,
            email: value.email,
            timezone: value.timezone,
            avatar_url: value.avatar_url,
        }
    }
}
