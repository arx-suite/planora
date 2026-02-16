use actix_web::{HttpRequest, Responder, get, post, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectRepo;
use super::model::{ProjectPriority, ProjectStatus, ProjectVisibility};
use crate::App;
use crate::common::{ApiError, ApiResult};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateProject {
    pub name: String,
    pub organization_id: Uuid,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub status: Option<ProjectStatus>,
    pub visibility: Option<ProjectVisibility>,
    pub priority: Option<ProjectPriority>,
    pub start_date: Option<DateTime<Utc>>,
    pub target_date: Option<DateTime<Utc>>,
}

#[utoipa::path(
    post,
    path = "/project",
    tag = "Workspace",
    request_body = CreateProject,
    responses(
        (status = 201, description = "Project has been created"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "project.create",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        name = payload.name,
        organization_id = payload.organization_id.to_string(),
        created_by = tracing::field::Empty
    )
)]
#[post("")]
async fn project_create(
    req: HttpRequest,
    app: web::Data<App>,
    payload: web::Json<CreateProject>,
) -> Result<impl Responder, ApiError> {
    let user = app.auth().extract_user_extension(&req)?;

    let pool = app.db().primary().await?;

    // TODO: user must have `project_create` permission
    let project = pool
        .project_create(
            /* TODO: replace this */ Uuid::new_v4(),
            payload.name.clone(),
            payload.description.clone(),
            payload.tags.clone(),
            payload.status.clone(),
            payload.visibility.clone(),
            payload.start_date,
            payload.target_date,
            user.user_id,
        )
        .await?;

    ApiResult::to_created_response("Project has been created", project)
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GetProject {
    organization_id: Uuid,
    project_id: Option<Uuid>,
    project_name: Option<String>,
}

#[utoipa::path(
    get,
    path = "/project",
    tag = "Workspace",
    request_body = GetProject,
    responses(
        (status = 200, description = "Project has been found"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "project.get",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        organization_id = payload.organization_id.to_string(),
        project_name = tracing::field::Empty,
        project_id = tracing::field::Empty
    )
)]
#[get("")]
async fn project_get(
    app: web::Data<App>,
    payload: web::Json<GetProject>,
) -> Result<impl Responder, ApiError> {
    // TODO: permission check via middleware / RBAC

    let pool = app.db().read().await?;

    let project = if let Some(project_id) = payload.project_id {
        tracing::Span::current().record("project_id", &project_id.to_string());

        pool.project_find_by_id(project_id).await?
    } else if let Some(ref name) = payload.project_name {
        tracing::Span::current().record("project_name", name);

        pool.project_find_by_name(name.clone(), payload.organization_id)
            .await?
    } else {
        return Err(ApiError::bad_request(
            "either project_id or project_name must be provided",
        ));
    };

    let project = project.ok_or_else(|| ApiError::not_found("project not found"))?;

    ApiResult::to_created_response("Project has been found", project)
}
