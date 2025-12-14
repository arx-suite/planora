use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::UserRow;

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    user_id: Uuid,
    user_tag: Option<String>,
    username: String,
    email: String,
    timezone: Option<String>,
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
