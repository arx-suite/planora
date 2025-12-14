use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProjectRow {
    pub project_id: Uuid,
    pub organization_id: Uuid,
    pub space_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sea_query::Iden)]
pub enum Projects {
    Table,
    ProjectId,
    OrganizationId,
    SpaceId,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}
