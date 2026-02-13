pub mod bootstrap;
pub mod common;
pub mod components;

pub use bootstrap::App;
pub use bootstrap::config::AppConfig;
pub use bootstrap::services;
pub use bootstrap::telemetry;

use components::user::handlers::{auth, oauth, profile};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        auth::signup,
        auth::verify_email,
        oauth::oauth_start,
        profile::get_profile
    ),
    components(
        schemas(
            auth::CreateUser,
            auth::VerifyEmail,
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
