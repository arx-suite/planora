pub mod bootstrap;
pub mod common;
pub mod components;

pub use bootstrap::App;
pub use bootstrap::config::AppConfig;
pub use bootstrap::services;
pub use bootstrap::telemetry;

use components::user::handlers::{auth, oauth};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        auth::signup,
        auth::verify_email,
        oauth::oauth_start
    ),
    components(schemas(auth::CreateUser, auth::VerifyEmail)),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;
