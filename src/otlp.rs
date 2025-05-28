use std::{borrow::Cow, env};

use fastrace::collector::Config;
use fastrace_opentelemetry::OpenTelemetryReporter;
use opentelemetry::{InstrumentationScope, KeyValue, trace::SpanKind};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) fn init_tracer() {
    let otel_endpoint =
        env::var("OTEL_ENDPOINT").unwrap_or("http://localhost:4317/v1/traces".to_string());
    let otel_service_name = env::var("OTEL_SERVICE_NAME").unwrap_or("saasexpress".to_string());

    info!("OTEL_ENDPOINT: {}", otel_endpoint);
    info!("OTEL_SERVICE_NAME: {}", otel_service_name);

    let reporter = OpenTelemetryReporter::new(
        SpanExporter::builder()
            .with_tonic()
            .with_endpoint(otel_endpoint)
            .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
            .with_timeout(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
            .build()
            .expect("initialize oltp exporter"),
        SpanKind::Server,
        Cow::Owned(
            Resource::builder()
                .with_attributes([KeyValue::new("service.name", otel_service_name)])
                .build(),
        ),
        InstrumentationScope::builder("example-crate")
            .with_version(env!("CARGO_PKG_VERSION"))
            .build(),
    );

    fastrace::set_reporter(reporter, Config::default());
}

pub(crate) fn init_logs() {
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| {
            "saasexpress_tenants=warn,saasexpress_core=debug,saasexpress=debug,tower_http=info".into()
        }),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();
}
