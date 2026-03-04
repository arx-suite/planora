use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::components::user::handlers::{auth, oauth, profile};
use crate::components::workspace::handlers::model as workspace_model;
use crate::components::workspace::handlers::organization;

// helper types
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResultEmpty {
    pub success: bool,
    pub message: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::signin,
        auth::signout,
        auth::signup,
        auth::verify_email,
        oauth::oauth_start,
        profile::get_profile
    ),
    components(
        schemas(
            // user profile
            auth::CreateUser,
            auth::VerifyEmail,
            auth::SigninPayload,
            oauth::OAuthProvider,
            profile::UserProfile,

            // workspace
            organization::CreateOrganization,
            workspace_model::OrganizationRow,
            workspace_model::OrganizationResourceRow,
            workspace_model::OrganizationFeatureRow
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Profile", description = "Managing profile"),
        (name = "Workspace", description = "Workspace")
    )
)]
pub struct ApiDoc;
