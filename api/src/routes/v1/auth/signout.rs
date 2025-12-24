use actix_web::{HttpRequest, HttpResponse, Responder, post};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::services::auth::cookie::expire_cookie;

#[post("/signout")]
#[tracing::instrument(
    name = "auth.signout",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty
    )
)]
async fn signout(req: HttpRequest) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::Span::current().record("user_id", &user_id.to_string());

    let (access_token_cookie, refresh_token_cookie) = expire_cookie();

    tracing::info!("signed out successfully");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResult::ok("signed out successfully")))
}

/*
#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};
    use arx_gatehouse::services::JwtService;

    use super::*;

    const JWT_SECRET: &'static str = "test_secret";
    const JWT_ACCESS_EXPIRY_MINUTES: i64 = 20;
    const JWT_REFRESH_EXPIRY_DAYS: i64 = 1;
    const USER_ID_1: uuid::Uuid = uuid::Uuid::from_u128(1);

    #[actix_web::test]
    async fn test_signout_success() {
        let jwt_service = JwtService::new(
            JWT_SECRET.to_string(),
            JWT_ACCESS_EXPIRY_MINUTES,
            JWT_REFRESH_EXPIRY_DAYS,
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(jwt_service.clone()))
                .service(signout),
        )
        .await;

        let (access_token, _) = jwt_service
            .generate_tokens(USER_ID_1)
            .expect("failed to generate tokens");

        let req = test::TestRequest::post()
            .uri("/signout")
            .cookie(
                actix_web::cookie::Cookie::build(JwtService::JWT_SESSION_KEY, access_token.clone())
                    .http_only(true)
                    .secure(false)
                    .finish(),
            )
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: ApiResult<()> = test::read_body_json(resp).await;
        assert!(body.success);
        assert_eq!(body.message, Some("logged out successfully".to_string()));
    }

    #[actix_web::test]
    async fn test_logout_without_cookie() {
        let jwt_service = JwtService::new(
            JWT_SECRET.to_string(),
            JWT_ACCESS_EXPIRY_MINUTES,
            JWT_REFRESH_EXPIRY_DAYS,
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(jwt_service.clone()))
                .service(signout),
        )
        .await;

        let req = test::TestRequest::post().uri("/signout").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);

        let body: ApiResult<()> = test::read_body_json(resp).await;
        assert!(!body.success);
        assert_eq!(body.message, Some("user is not authenticated".to_string()));
    }
}
*/
