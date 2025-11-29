use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub user_tag: Option<String>,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sea_query::Iden)]
pub enum Users {
    Table,
    UserId,
    UserTag,
    Username,
    Email,
    Password,
    Timezone,
    AvatarUrl,
    CreatedAt,
    UpdatedAt,
}
