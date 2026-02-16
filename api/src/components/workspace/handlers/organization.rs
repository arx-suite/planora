use actix_web::{HttpRequest, Responder, post, web};
use serde::Deserialize;

use super::WorkspaceRepo;
use crate::App;
use crate::common::{ApiError, ApiResult};

#[derive(Debug, Clone, Deserialize, utoipa::ToSchema)]
pub struct CreateOrganization {
    pub name: String,
    pub subdomain: String,
}

#[utoipa::path(
    post,
    path = "/organization",
    tag = "Workspace",
    request_body = CreateOrganization,
    responses(
        (status = 201, description = "Organization has been created"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "organization.create",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        name = payload.name,
        organization_id = tracing::field::Empty,
        created_by = tracing::field::Empty
    )
)]
#[post("")]
async fn organization_create(
    req: HttpRequest,
    app: web::Data<App>,
    payload: web::Json<CreateOrganization>,
) -> Result<impl Responder, ApiError> {
    let user = app.auth().extract_user_extension(&req)?;
    tracing::Span::current().record("created_by", &user.user_id.to_string());

    let pool = app.db().primary().await?;
    tracing::debug!("database pool acquired");

    let organization = pool
        .organization_create(
            user.user_id,
            payload.name.clone(),
            payload.subdomain.clone(),
        )
        .await?;

    tracing::Span::current().record("created_by", &organization.organization_id.to_string());

    ApiResult::to_created_response("Organization has been created", organization)
}
