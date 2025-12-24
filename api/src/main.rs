/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    Copyright (C) 2025 Planora
*/

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, middleware, web};

use arx_gatehouse::{common::ApiResult, services, telemetry};

use crate::routes::v1::v1_scope;

mod config;
mod middlewares;
mod routes;
mod ws;

pub const fn public_paths() -> [&'static str; 4] {
    [
        "/v1/auth/signin",
        "/v1/auth/signup",
        "/v1/auth/refresh",
        "/v1/health",
    ]
}

async fn not_found_handler() -> HttpResponse {
    HttpResponse::NotFound().json(ApiResult::error("Endpoint not found"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let config = config::Config::from_env();

    // observability
    let _guard = telemetry::telemetry::init();

    // initialize the application
    let is_production_env = config.is_production_env();
    let web_url = config.next_base_url.to_owned();
    tracing::info!(
        "{} v{} initialized - running in {} ({}) mode",
        config.app_name,
        config.app_version,
        config.app_env,
        config.profile
    );

    /* services */
    let db_service = services::db::service::init().await;
    let cache_service = services::cache::service::init();
    let auth_service = services::auth::service::init();
    let s3_service = services::s3::service::init(config.app_name.clone()).await;

    // actix server
    tracing::info!("Starting server at http://{}", config.addr());
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
            .app_data(web::Data::new(cache_service.clone()))
            .app_data(web::Data::new(db_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(s3_service.clone()))
            .route("/ws", web::get().to(ws::ws))
            .service(v1_scope().wrap(middlewares::AuthMiddleware::new(
                public_paths().into(),
                auth_service.clone(),
                db_service.clone(),
            )))
            .default_service(web::to(not_found_handler))
    })
    .bind(config.addr())?
    .run()
    .await?;

    Ok(())
}
