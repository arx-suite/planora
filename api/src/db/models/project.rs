use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub project_id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}
