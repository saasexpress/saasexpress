use once_cell::sync::Lazy;
use opentelemetry::{
    Context, KeyValue, global,
    trace::{SpanKind, TraceContextExt, Tracer},
};
//use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::Protocol;
use opentelemetry_otlp::WithExportConfig;
//use //opentelemetry_otlp::WithHttpConfig;
//use opentelemetry_sdk::trace::BatchSpanProcessor;
use opentelemetry_sdk::{
    Resource,
    //logs::SdkLoggerProvider,
    propagation::TraceContextPropagator,
    trace::{RandomIdGenerator, Sampler},
};
//use opentelemetry_stdout::{LogExporter, SpanExporter};
use tracing::info;
use tracing_subscriber::Layer;
use tracing_subscriber::{
    EnvFilter, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

// Copyright The OpenTelemetry Authors
// SPDX-License-Identifier: Apache-2.0

use opentelemetry::trace::TraceError;
use opentelemetry_otlp;
use opentelemetry_sdk::{runtime, trace as sdktrace};
use tracing_subscriber::Registry;

use crate::res::get_resource_attr;

pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                //.with_endpoint("http://127.0.0.1:4318/v1/traces")
                .with_protocol(opentelemetry_otlp::Protocol::HttpBinary),
        )
        .with_trace_config(sdktrace::config().with_resource(get_resource_attr()))
        .install_batch(runtime::Tokio)
}

// pub fn init_reqwest_tracing(
//     tracer: sdktrace::Tracer,
// ) -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
//     let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
//     let subscriber = Registry::default().with(telemetry);
//     tracing::subscriber::set_global_default(subscriber)
// }

// static RESOURCE: Lazy<Resource> = Lazy::new(|| {
//     Resource::builder()
//         .with_service_name("basic-otlp-example-http")
//         .build()
// });

// pub(crate) fn init_tracer() -> SdkTracerProvider {
//     global::set_text_map_propagator(TraceContextPropagator::new());

//     let client = reqwest::Client::new();

//     let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
//         // .with_tonic()
//         .with_http()
//         .with_http_client(client)
//         .with_protocol(Protocol::HttpBinary)
//         .with_endpoint("http://localhost:4318/v1/traces")
//         .build()
//         .expect("Failed to create OTLP exporter");

//     // Install stdout exporter pipeline to be able to retrieve the collected spans.
//     // For the demonstration, use `Sampler::AlwaysOn` sampler to sample all traces.
//     let provider = SdkTracerProvider::builder()
//         .with_simple_exporter(otlp_exporter)
//         .with_resource(RESOURCE.clone())
//         //.with_span_processor(BatchSpanProcessor::builder(otlp_exporter).build())
//         //.with_simple_exporter(otlp_exporter)
//         .with_sampler(Sampler::AlwaysOn)
//         .with_id_generator(RandomIdGenerator::default())
//         .with_max_events_per_span(64)
//         .with_max_attributes_per_span(16)
//         .with_resource(
//             Resource::builder()
//                 .with_attributes(vec![KeyValue::new(
//                     "service.name",
//                     "saasexpress".to_string(),
//                 )])
//                 .build(),
//         )
//         .build();

//     global::set_tracer_provider(provider.clone());
//     provider
// }

// pub(crate) fn init_logs() -> SdkLoggerProvider {
//     // Setup tracerprovider with stdout exporter
//     // that prints the spans to stdout.
//     let logger_provider = SdkLoggerProvider::builder()
//         .with_simple_exporter(LogExporter::default())
//         .build();

//     let filter =
//         tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| {
//             "saasexpress_tenants=debug,saasexpress_core=info,saasexpress=debug,tower_http=error"
//                 .into()
//         }));

//     let filter_otel = EnvFilter::new("info")
//         .add_directive("hyper=off".parse().unwrap())
//         //        .add_directive("opentelemetry=off".parse().unwrap())
//         .add_directive("tonic=off".parse().unwrap())
//         .add_directive("h2=off".parse().unwrap())
//         .add_directive("reqwest=off".parse().unwrap());

//     let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);

//     // tracing_subscriber::registry()
//     //     .with(filter)
//     //     .with(telemetry)
//     //     .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
//     //     .init();
//     //let tracer = provider.tracer("graph_runtime");

//     // let tracer = global::tracer("runtime");
//     // //Create a tracing layer with the configured tracer
//     // let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

//     let exporter = opentelemetry_stdout::LogExporter::default();
//     let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
//         .with_resource(
//             Resource::builder()
//                 .with_service_name("log-appender-tracing-example")
//                 .build(),
//         )
//         .with_simple_exporter(exporter)
//         .build();
//     let filter_otel = EnvFilter::new("info")
//         .add_directive("hyper=off".parse().unwrap())
//         .add_directive("opentelemetry=off".parse().unwrap())
//         .add_directive("tonic=off".parse().unwrap())
//         .add_directive("h2=off".parse().unwrap())
//         .add_directive("reqwest=off".parse().unwrap());
//     let otel_layer = OpenTelemetryTracingBridge::new(&provider).with_filter(filter_otel);

//     let filter_fmt = EnvFilter::new("info").add_directive("opentelemetry=debug".parse().unwrap());
//     let fmt_layer = tracing_subscriber::fmt::layer()
//         .with_thread_names(true)
//         .with_filter(filter_fmt);

//     tracing_subscriber::registry()
//         .with(otel_layer)
//         .with(fmt_layer)
//         .init();
//     // tracing_subscriber::registry()
//     //     .with(filter)
//     //     .with(filter_otel)
//     //     .with(otel_layer)
//     //     .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
//     //     .with(tracing_subscriber::filter::LevelFilter::INFO)
//     //     .init();

//     logger_provider
// }
