use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrganizationRow {
    pub organization_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub subdomain: String,
    pub plan: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sea_query::Iden)]
pub enum Organizations {
    Table,
    OrganizationId,
    OwnerId,
    Name,
    Subdomain,
    Plan,
    CreatedAt,
    UpdatedAt,
}
