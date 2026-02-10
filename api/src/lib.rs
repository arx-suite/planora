pub mod bootstrap;
pub mod common;
pub mod components;

pub use bootstrap::App;
pub use bootstrap::config::AppConfig;
pub use bootstrap::services;
pub use bootstrap::telemetry;

use components::user::handlers::auth;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(auth::signup, auth::verify_email),
    components(schemas(auth::CreateUser, auth::VerifyEmail)),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;
