use actix_web::{Responder, post, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::TaskRepo;
use super::model::{TaskPriority, TaskStatus};
use crate::App;
use crate::common::{ApiError, ApiResult};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTask {
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
}

#[utoipa::path(
    post,
    path = "/task",
    tag = "Workspace",
    request_body = CreateTask,
    responses(
        (status = 201, description = "Task has been created"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "task.create",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        project_id = payload.project_id.to_string(),
        task_name = payload.name
    )
)]
#[post("")]
async fn task_create(
    app: web::Data<App>,
    payload: web::Json<CreateTask>,
) -> Result<impl Responder, ApiError> {
    let pool = app.db().primary().await?;
    let task = pool
        .task_create(
            payload.project_id,
            payload.name.clone(),
            payload.description.clone(),
            payload.status.clone(),
            payload.priority.clone(),
            payload.start_date,
            payload.due_date,
        )
        .await?;

    ApiResult::to_created_response("Task has been created", task)
}
