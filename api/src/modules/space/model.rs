use chrono::{DateTime, Utc};
use sea_query::Iden;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SpaceRow {
    pub space_id: Uuid,
    pub organization_id: Uuid,
    pub space_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
}
