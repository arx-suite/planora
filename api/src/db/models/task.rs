use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    #[default]
    NotStarted,
    Active,
    InReview,
    Archived,
    Done,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_priority", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Eliminate,
    Delegate,
    Schedule,
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub task_name: String,
    pub description: Option<String>,
    pub r#type: String,
    pub assignor: Uuid,
    pub assignee: Uuid,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress: i8,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskComment {
    pub comment_id: Uuid,
    pub task_id: Uuid,
    pub author: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
