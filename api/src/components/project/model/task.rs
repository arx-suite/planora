use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct TaskRow {
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub parent_id: Uuid,

    pub task_key: String,
    pub task_name: String,
    pub description: Option<String>,
    pub r#type: String,

    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub tags: Vec<String>,

    pub estimated_hours: Option<sqlx::types::Decimal>,
    pub actual_hours: Option<sqlx::types::Decimal>,
    pub remaining_hours: Option<sqlx::types::Decimal>,

    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,

    pub progress: i16,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sea_query::Iden)]
pub enum Tasks {
    Table,
    TaskId,
    ProjectId,
    ParentId,
    TaskKey,
    TaskName,
    Description,
    Type,
    Status,
    Priority,
    Tags,
    EstimatedHours,
    ActualHours,
    RemainingHours,
    StartDate,
    DueDate,
    CompletedAt,
    Progress,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    #[default]
    Backlog,
    Planned,
    InProgress,
    InReview,
    Blocked,
    Done,
    Archived,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "task_priority", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum TaskPriority {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}
