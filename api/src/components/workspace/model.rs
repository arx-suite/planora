use chrono::{DateTime, Utc};
use serde::Serialize;

// organization
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct OrganizationRow {
    pub organization_id: uuid::Uuid,
    pub created_by: uuid::Uuid,
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
    CreatedBy,
    Name,
    Subdomain,
    Plan,
    CreatedAt,
    UpdatedAt,
}

// resources, and features
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct OrganizationResourceRow {
    pub resource_key: String,
    pub unit: ResourceUnit,
    pub description: Option<String>,
    pub soft_limit: i64,
    pub hard_limit: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct OrganizationFeatureRow {
    pub feature_name: String,
    pub description: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "resource_unit", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum ResourceUnit {
    Bytes,
    Mb,
    Gb,
    Requests,
    Minutes,
}

#[derive(sea_query::Iden)]
pub enum Plans {
    Table,
    PlanName,
    PlanLevel,
    Description,
    IsMetered,
    CreatedAt,
}

#[derive(sea_query::Iden)]
pub enum PlanQuotas {
    Table,
    PlanName,
    QuotaKey,
    QuotaValue,
    Description,
}

#[derive(sea_query::Iden)]
pub enum PlanResources {
    Table,
    PlanName,
    Description,
    Unit,
    ResourceKey,
    SoftLimit,
    HardLimit,
}

#[derive(sea_query::Iden)]
pub enum Features {
    Table,
    FeatureName,
    Description,
    MinPlanLevel,
    DefaultEnabled,
    CreatedAt,
}
