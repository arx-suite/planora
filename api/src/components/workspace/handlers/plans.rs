use actix_web::{HttpRequest, Responder, get, web};

use super::WorkspaceRepo;
use super::model::OrganizationFeatureRow;

use crate::App;
use crate::common::{ApiError, ApiResult};
use crate::context::tenant::TenantContext;
use crate::doc::ApiResultEmpty;

#[utoipa::path(
    get,
    path = "/features",
    tag = "Workspace",
    responses(
        (status = 200, description = "Features List", body = ApiResult<OrganizationFeatureRow>),
        (status = 500, description = "Internal server error", body = ApiResultEmpty)
    )
)]
#[tracing::instrument(
    name = "organization.features",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        organization_id = tracing::field::Empty,
    )
)]
#[get("/features")]
async fn features_get(req: HttpRequest, app: web::Data<App>) -> Result<impl Responder, ApiError> {
    let tenant = TenantContext::extract(&req)?;
    tracing::Span::current().record("organization_id", &tenant.organization_id.to_string());

    let pool = app.db().read().await?;
    tracing::debug!("database pool acquired");

    let features = pool.features(tenant.organization_id).await?;
    tracing::debug!(len = %features.len(), "features list");

    ApiResult::to_ok_response("Features List", features)
}
