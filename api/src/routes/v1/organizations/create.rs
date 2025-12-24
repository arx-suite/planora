use actix_web::{HttpRequest, Responder, post, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::modules::organization::{CreateOrg, OrgProfile, OrgRepo};
use arx_gatehouse::services::DbService;

#[post("")]
#[tracing::instrument(
    name = "org.create",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty,
        org_id = tracing::field::Empty
    )
)]
async fn create_organization(
    db_service: web::Data<DbService>,
    req: HttpRequest,
    payload: web::Json<CreateOrg>,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::Span::current().record("user_id", &user_id.to_string());

    let create_org = payload.into_inner();

    let pool = db_service.primary().await?;
    tracing::debug!("database pool acquired");

    let inserted_org = pool.org_create(&create_org, user_id).await?;
    tracing::Span::current().record("org_id", &inserted_org.organization_id.to_string());

    tracing::info!("organization created successfully");

    ApiResult::to_ok_response(
        "organization has been created",
        OrgProfile::from(inserted_org),
    )
}
