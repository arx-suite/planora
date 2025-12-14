use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRow {
    pub user_id: Uuid,
    pub user_tag: Option<String>,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub timezone: Option<String>,
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
