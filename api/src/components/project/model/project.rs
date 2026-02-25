use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct ProjectRow {
    pub project_id: Uuid,
    pub organization_id: Uuid,

    // metadata
    pub project_name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,

    // status
    pub status: ProjectStatus,
    pub visibility: ProjectVisibility,
    pub priority: ProjectPriority,

    // timeline
    pub start_date: Option<DateTime<Utc>>,
    pub target_date: Option<DateTime<Utc>>,
    pub actual_end_date: Option<DateTime<Utc>>,

    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sea_query::Iden)]
pub enum Projects {
    Table,
    ProjectId,
    OrganizationId,
    ProjectName,
    Description,
    Tags,
    Status,
    Visibility,
    Priority,
    StartDate,
    TargetDate,
    ActualEndDate,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "project_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum ProjectStatus {
    #[default]
    Planned,
    Active,
    OnHold,
    Completed,
    Archived,
}

impl From<ProjectStatus> for sea_query::Value {
    fn from(value: ProjectStatus) -> Self {
        match value {
            ProjectStatus::Planned => "planned".into(),
            ProjectStatus::Active => "active".into(),
            ProjectStatus::OnHold => "on_hold".into(),
            ProjectStatus::Completed => "completed".into(),
            ProjectStatus::Archived => "archived".into(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "project_visibility", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum ProjectVisibility {
    Private,
    #[default]
    Team,
    Public,
}

impl From<ProjectVisibility> for sea_query::Value {
    fn from(value: ProjectVisibility) -> Self {
        match value {
            ProjectVisibility::Private => "private".into(),
            ProjectVisibility::Team => "team".into(),
            ProjectVisibility::Public => "public".into(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "project_priority", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum ProjectPriority {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

impl From<ProjectPriority> for sea_query::Value {
    fn from(value: ProjectPriority) -> Self {
        match value {
            ProjectPriority::Low => "low".into(),
            ProjectPriority::Medium => "medium".into(),
            ProjectPriority::High => "high".into(),
            ProjectPriority::Critical => "critical".into(),
        }
    }
}
