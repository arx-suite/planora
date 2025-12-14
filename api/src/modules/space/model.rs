use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Space {
    pub space_id: Uuid,
    pub organization_id: Uuid,
    pub space_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Iden)]
pub enum Spaces {
    Table,
    OrganizationId,
    SpaceId,
    SpaceName,
    Description,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
