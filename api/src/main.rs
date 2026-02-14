/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    Copyright (C) 2025 Planora
*/

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, middleware, web};

use arx_gatehouse::doc::ApiDoc;
use arx_gatehouse::{bootstrap, common::ApiResult, components};
use utoipa::OpenApi;

mod middlewares;
mod ws;

pub fn v1_scope() -> actix_web::Scope {
    web::scope("/v1")
        .service(components::user::handlers::auth_scope())
        .service(components::user::handlers::profile_scope())
}

pub const fn public_paths() -> [&'static str; 3] {
    [
        "/v1/auth/signin",
        "/v1/auth/signup",
        "/v1/auth/verify-email",
    ]
}

async fn not_found_handler() -> HttpResponse {
    HttpResponse::NotFound().json(ApiResult::error("Endpoint not found"))
}

#[actix_web::get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResult::ok("Ok"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // bootstrap
    let (app, _guard) = bootstrap::init().await;

    let addr = app.config().addr();
    let is_production_env = app.config().is_production_env();
    let web_url = app.config().web_url.clone();

    tracing::info!("Starting server at http://{}", addr);
    HttpServer::new(move || {
        use actix_web::http::header;

        // For development: allow all cross-origin requests (CORS)
        // In production, restrict CORS to specific allowed origins for security.
        let cors = if !is_production_env {
            Cors::permissive()
        } else {
            Cors::default()
                .allowed_origin(&web_url)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ])
                .max_age(3600)
        };

        App::new()
            .wrap(opentelemetry_instrumentation_actix_web::RequestTracing::new())
            .wrap(opentelemetry_instrumentation_actix_web::RequestMetrics::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(cors)
            .app_data(web::Data::new(app.clone()))
            .route("/ws", web::get().to(ws::ws))
            .service(v1_scope().wrap(middlewares::AuthMiddleware::new(
                public_paths().into_iter().collect::<Vec<_>>(),
            )))
            .service(health_check)
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .default_service(web::to(not_found_handler))
    })
    .bind(addr.clone())?
    .run()
    .await?;

    Ok(())
}
