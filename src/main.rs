use std::borrow::Cow;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use bootstrap::build_graph;
use commands::config::config;
use commands::samples::samples;
use commands::stdin::stdin;
use commands::{args::parse_commands, get::get};
use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use fastrace::prelude::*;
use fastrace_opentelemetry::OpenTelemetryReporter;
use futures::channel::oneshot;
use logs::init_logger;
//use opentelemetry_otlp::WithHttpConfig;
//use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{BatchConfig, RandomIdGenerator, Sampler};
use otlp::{init_logs, init_tracer};
use saasexpress_tenants::TenantsService;
use tracing::{error, info, info_span, span};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod bootstrap;
mod commands;
mod operators;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::trace::{SpanKind, Tracer};
use opentelemetry::{Context, InstrumentationScope, global};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::{Protocol, SpanExporter};

use opentelemetry_http::{Bytes, HeaderInjector};

use opentelemetry::KeyValue;
use opentelemetry::trace::FutureExt;
use opentelemetry::trace::TraceContextExt;
use opentelemetry_sdk::{Resource, trace};
use tracing::Instrument;

mod logs;
mod otlp;
mod res;

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
//#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let matches = parse_commands();

    // Initialize reporter
    let reporter = OpenTelemetryReporter::new(
        SpanExporter::builder()
            .with_tonic()
            .with_endpoint("http://localhost:4317/v1/traces".to_string())
            .with_protocol(opentelemetry_otlp::Protocol::Grpc)
            .with_timeout(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
            .build()
            .expect("initialize oltp exporter"),
        SpanKind::Server,
        Cow::Owned(
            Resource::builder()
                .with_attributes([KeyValue::new("service.name", "saasexpress")])
                .build(),
        ),
        InstrumentationScope::builder("example-crate")
            .with_version(env!("CARGO_PKG_VERSION"))
            .build(),
    );

    fastrace::set_reporter(reporter, Config::default());

    {
        // Start tracing
        let root = Span::root("root", SpanContext::random());

        root.add_event(Event::new("event in root"));

        async move {
            let _span1 = LocalSpan::enter_with_local_parent("a child span");

            LocalSpan::add_event(Event::new("event in span1"));
        }
        .in_span(root)
        .await;
    }

    //fastrace::flush();

    //init_tracer();
    init_logs();
    //let logger_provider = init_logs();

    // tracing_subscriber::registry()
    // .with(tracing_subscriber::EnvFilter::new(
    //     std::env::var("RUST_LOG").unwrap_or_else(|_| {
    //         "saasexpress_tenants=debug,saasexpress_core=info,saasexpress=debug,tower_http=info".into()
    //     }),
    // ))
    // .with(tracing_subscriber::fmt::layer())
    // .init();

    // opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    // let client = reqwest::Client::new();
    // let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
    //     //.with_tonic()
    //     .with_http()
    //     .with_http_client(client)
    //     .with_protocol(Protocol::HttpBinary)
    //     .with_endpoint("http://localhost:4318/v1/traces")
    //     .build()?;

    // // // Create a tracer provider with the exporter
    // let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
    //     .with_simple_exporter(otlp_exporter)
    //     //.with_batch_exporter(otlp_exporter)
    //     .with_sampler(Sampler::AlwaysOn)
    //     .with_id_generator(RandomIdGenerator::default())
    //     .with_max_events_per_span(64)
    //     .with_max_attributes_per_span(16)
    //     .with_resource(
    //         Resource::builder()
    //             .with_attributes(vec![KeyValue::new(
    //                 "service.name",
    //                 "saasexpress".to_string(),
    //             )])
    //             .build(),
    //     )
    //     .build();
    // //    let tracer = provider.tracer("saasexpress");

    // global::set_tracer_provider(provider.clone());

    // Get a tracer and create spans
    // let tracer = global::tracer("saaasexpress_trace");
    // tracer.in_span("doing_work", |_cx| {
    //     // Your application logic here...
    //     sleep(Duration::from_secs(2));
    //     info!("doing work");
    // });

    // // Get filter based on RUST_LOG env var
    // let filter =
    //     tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| {
    //         "saasexpress_tenants=debug,saasexpress_core=info,saasexpress=debug,tower_http=error,reqwest=error,hyper=error"
    //             .into()
    //     }));

    // let filter_otel = EnvFilter::new("info")
    //     .add_directive("hyper=off".parse().unwrap())
    //     .add_directive("opentelemetry=off".parse().unwrap())
    //     .add_directive("tonic=off".parse().unwrap())
    //     .add_directive("h2=off".parse().unwrap())
    //     .add_directive("reqwest=off".parse().unwrap());
    //let otel_layer = otel_layer.with_filter(filter_otel);

    // Configure the tracing subscriber with OpenTelemetry
    //let tracer = provider.tracer("graph_runtime");

    // Create a tracing layer with the configured tracer
    // let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // // Use the tracing subscriber `Registry`, or any other subscriber
    // // that impls `LookupSpan`
    // let subscriber = Registry::default()
    //     .with(filter)
    //     .with(filter_otel)
    //     .with(telemetry)
    //     .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE));

    //subscriber.with(filter);

    // tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    // Trace executed code
    // tracing::subscriber::with_default(subscriber, || {
    //     // Spans will be sent to the configured OpenTelemetry exporter
    //     let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    //     let _enter = root.enter();

    //     error!("This event will be logged in the root span.");
    // });
    // tracing_subscriber::registry()
    //     .with(filter)
    //     .with(telemetry)
    //     .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
    //     .init();

    // // Create a guard that ensures spans are flushed on shutdown
    // let _guard = telemetry::create_guard();

    // async move {
    //     let span_name = "abc";
    //     let url = "http://localhost:8080/tenants";
    //     let tracer = global::tracer("example/client");
    //     let span = tracer
    //         .span_builder(String::from(span_name))
    //         .with_kind(SpanKind::Client)
    //         .start(&tracer);
    //     let cx = Context::current_with_span(span);

    //     let mut req = hyper::Request::builder().uri(url);
    //     global::get_text_map_propagator(|propagator| {
    //         propagator.inject_context(&cx, &mut HeaderInjector(req.headers_mut().unwrap()))
    //     });
    // }
    // .await;

    if matches.get_flag("stdin") {
        stdin();
    }
    if matches.get_flag("samples") {
        samples();
    }

    tokio::spawn(TenantsService::start());

    // get config file

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        config(config_path.to_str().unwrap().to_string());
    }
    // let config = matches.get_one::<String>("config").unwrap();
    // println!("Config file: {:?}", config);
    // if matches.get_flag("config") {
    //     let config = matches.get_one::<String>("config").unwrap();
    //     println!("Config file: {:?}", config);
    // }
    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            get(sub_matches);
        }
        _ => {}
    }

    {
        let start = Span::root("start_up", SpanContext::random());

        let _guard = start.set_local_parent();

        TenantsService::saasexpress_graphs()
            .iter()
            .for_each(|yaml| build_graph(yaml.to_owned()));

        bootstrap::bootstrap();
    }

    do_it();

    loop {
        const ONE_HOUR: u64 = 3600;
        std::thread::sleep(std::time::Duration::from_secs(ONE_HOUR));
    }

    // Shutdown trace pipeline
    // tracer_provider
    //     .shutdown()
    //     .expect("TracerProvider should shutdown successfully");
    // logger_provider
    //     .shutdown()
    //     .expect("LoggerProvider should shutdown successfully");
}

#[derive(Debug)]
pub struct Origin<T> {
    context: T,
}

impl<T> Origin<T>
where
    T: Send + Sync,
{
    pub fn new(context: T) -> Self {
        Origin { context }
    }
}

#[derive(Debug)]
pub enum XMessage {
    Split {
        message: Vec<u8>,
        origin: Origin<String>,
    },
    Standard {
        message: Vec<u8>,
        origin: Origin<oneshot::Sender<XMessage>>,
    },
}

fn do_it() {
    // Example usage
    let chnl = oneshot::channel();
    let list_messages = vec![
        XMessage::Split {
            message: vec![1, 2, 3],
            origin: Origin::new("example_context".to_string()),
        },
        XMessage::Standard {
            message: vec![4, 5, 6],
            origin: Origin::new(chnl.0),
        },
    ];
    for message in list_messages {
        match message {
            XMessage::Split { message, origin } => {
                println!("Split Message: {:?}", message);
                println!("Origin: {:?}", origin.context);
            }
            XMessage::Standard { message, origin } => {
                println!("Standard Message: {:?}", message);
                println!("Origin: {:?}", origin.context);
            }
        }
    }
}
