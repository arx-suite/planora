use actix_web::{HttpRequest, Responder, post, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::project::{CreateProject, ProjectInfo, ProjectRepo};
use arx_gatehouse::services::DbService;

#[post("")]
async fn create_project(
    db_service: web::Data<DbService>,
    payload: web::Json<CreateProject>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let project = payload.into_inner();

    let pool = db_service.primary().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%project.name, %org_id, "create project for organization");

    let inserted_project: ProjectInfo = pool.project_create(&project, org_id).await?.into();

    tracing::info!(%inserted_project.project_id, %org_id, "project created successfully");

    ApiResult::to_ok_response("project has been created successfully", inserted_project)
}
