use opentelemetry::global;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

const SERVICE_NAME: &'static str = "arx-gatehouse";

pub struct ObservabilityGuard {
    logger_provider: SdkLoggerProvider,
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
}

impl Drop for ObservabilityGuard {
    fn drop(&mut self) {
        let _ = self.logger_provider.shutdown();
        let _ = self.tracer_provider.shutdown();
        let _ = self.meter_provider.shutdown();
    }
}

pub fn init() -> ObservabilityGuard {
    let resource = Resource::builder().with_service_name(SERVICE_NAME).build();

    // logs
    let log_exporter = LogExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create log exporter");

    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(log_exporter)
        .build();

    let otel_log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    // traces
    let span_exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create span exporter");

    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(span_exporter)
        .build();

    global::set_tracer_provider(tracer_provider.clone());

    let tracer = global::tracer(SERVICE_NAME);
    let otel_trace_layer = OpenTelemetryLayer::new(tracer);

    // metrics
    let metric_exporter = MetricExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create metric exporter");

    let meter_provider = SdkMeterProvider::builder()
        .with_periodic_exporter(metric_exporter)
        .with_resource(resource)
        .build();

    global::set_meter_provider(meter_provider.clone());

    // subscriber
    let env_filter = EnvFilter::from_default_env()
        .add_directive("actix_server=warn".parse().unwrap())
        .add_directive("actix_web=warn".parse().unwrap())
        .add_directive("actix_http=warn".parse().unwrap())
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("mio=off".parse().unwrap())
        .add_directive("aws_runtime=off".parse().unwrap())
        .add_directive("aws_smithy_runtime=off".parse().unwrap())
        .add_directive("aws_config=off".parse().unwrap())
        .add_directive("tower=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());

    let filter_fmt = EnvFilter::new("info");
    let fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter_fmt);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(otel_trace_layer)
        .with(otel_log_layer)
        .init();

    ObservabilityGuard {
        logger_provider,
        tracer_provider,
        meter_provider,
    }
}
