use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::components::user::handlers::{auth, oauth, profile};

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
            auth::CreateUser,
            auth::VerifyEmail,
            auth::SigninPayload,
            oauth::OAuthProvider,
            profile::UserProfile
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Profile", description = "Managing profile")
    )
)]
pub struct ApiDoc;
