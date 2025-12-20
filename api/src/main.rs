/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    Copyright (C) 2025 Planora
*/

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, middleware, web};

use opentelemetry::global;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

use arx_gatehouse::{common::ApiResult, services};

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
    let logger_provider = config::init_logs();

    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    let filter_otel = EnvFilter::from_default_env()
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());

    let otel_layer = otel_layer.with_filter(filter_otel);

    let filter_fmt = EnvFilter::new("info").add_directive("opentelemetry=debug".parse().unwrap());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();

    let tracer_provider = config::init_traces();
    global::set_tracer_provider(tracer_provider.clone());

    let meter_provider = config::init_metrics();
    global::set_meter_provider(meter_provider.clone());

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
    // database
    let manager = services::DbManager::new();
    manager
        .init_pool()
        .await
        .expect("Failed to connect to Postgres");

    // auth
    let auth_service = services::AuthService::from_env();

    // bucket storage
    let bucket_service = services::S3Service::from_env(config.app_name.clone())
        .await
        .expect("Failed to setup S3");

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
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(cors)
            .app_data(web::Data::new(manager.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(bucket_service.clone()))
            .route("/ws", web::get().to(ws::ws))
            .service(v1_scope().wrap(middlewares::AuthMiddleware::new(
                public_paths().into(),
                auth_service.clone(),
                manager.clone(),
            )))
            .default_service(web::to(not_found_handler))
    })
    .bind(config.addr())?
    .run()
    .await?;

    // shutdown
    let mut shutdown_errors = Vec::new();
    if let Err(e) = tracer_provider.shutdown() {
        shutdown_errors.push(format!("tracer provider: {e}"));
    }

    if let Err(e) = meter_provider.shutdown() {
        shutdown_errors.push(format!("meter provider: {e}"));
    }

    if let Err(e) = logger_provider.shutdown() {
        shutdown_errors.push(format!("logger provider: {e}"));
    }

    if !shutdown_errors.is_empty() {
        return Err(std::io::Error::other(format!(
            "Failed to shutdown providers:{}",
            shutdown_errors.join("\n")
        )));
    }

    Ok(())
}
