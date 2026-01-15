/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    Copyright (C) 2025 Planora
*/

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, middleware, web};

use arx_gatehouse::{bootstrap, common::ApiResult, telemetry};

use crate::routes::v1::v1_scope;

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
    // bootstrap
    let app = bootstrap::init().await;

    let _guard = telemetry::telemetry::init();

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
                public_paths().into(),
                app.auth().clone(),
                app.db().clone(),
            )))
            .default_service(web::to(not_found_handler))
    })
    .bind(addr.clone())?
    .run()
    .await?;

    Ok(())
}
