use actix_web::{HttpRequest, Responder, delete, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::project::{DeleteProject, ProjectRepo};
use arx_gatehouse::services::DbService;

#[delete("")]
async fn delete_project(
    db_service: web::Data<DbService>,
    payload: web::Json<DeleteProject>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let project = payload.into_inner();

    let pool = db_service.primary().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%project.project_id, %org_id, "delete project");

    let affected_rows = pool
        .project_delete_by_id(project.clone(), org_id)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, %project.project_id, %org_id, "Failed to delete project");
            ApiError::Internal("Error deleting project".into())
        })?;

    if affected_rows == 0 {
        tracing::warn!(%project.project_id, %org_id, "No project found to delete");
        return ApiResult::to_not_found("Project not found");
    }

    tracing::info!(%project.project_id, %org_id, %affected_rows, "Project deleted successfully");

    ApiResult::to_ok_response("Project has been deleted successfully", affected_rows)
}
