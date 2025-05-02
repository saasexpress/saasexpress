use opentelemetry::sdk::{Resource, trace};
use opentelemetry::{KeyValue, global};
use opentelemetry_jaeger::Propagator;
use opentelemetry_otlp::WithExportConfig;
use std::env;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::Registry;

/// Initialize the OpenTelemetry exporter.
pub fn init_telemetry(service_name: &str) {
    // Set the Jaeger propagator as the global propagator
    global::set_text_map_propagator(Propagator::new());

    // Check if Jaeger or OTLP is configured
    if let Ok(jaeger_endpoint) = env::var("JAEGER_ENDPOINT") {
        init_jaeger(service_name, &jaeger_endpoint);
    } else if let Ok(otlp_endpoint) = env::var("OTLP_ENDPOINT") {
        init_otlp(service_name, &otlp_endpoint);
    } else {
        // Default to Jaeger on localhost if no environment variables are set
        init_jaeger(service_name, "http://localhost:14268/api/traces");
    }
}

fn init_jaeger(service_name: &str, endpoint: &str) {
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_collector_endpoint(endpoint)
        .install_simple()
        .expect("Failed to install Jaeger tracer");

    global::set_tracer_provider(tracer);
}

fn init_otlp(service_name: &str, endpoint: &str) {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_endpoint(endpoint),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                service_name.to_string(),
            )])),
        )
        .install_simple()
        .expect("Failed to install OTLP tracer");

    global::set_tracer_provider(tracer);
}

/// Create an OpenTelemetry layer for the tracing subscriber
pub fn get_otel_layer<S>() -> OpenTelemetryLayer<S, opentelemetry::sdk::trace::Tracer>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    tracing_opentelemetry::layer().with_tracer(global::tracer("saasexpress"))
}

// Ensure we flush the OpenTelemetry pipeline when the program exits
pub struct TracingGuard;

impl Drop for TracingGuard {
    fn drop(&mut self) {
        // Flush remaining spans
        global::shutdown_tracer_provider();
    }
}

// Create a guard that will flush the tracer when dropped
pub fn create_guard() -> TracingGuard {
    TracingGuard
}
