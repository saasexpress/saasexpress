# SaaSExpress OpenTelemetry Integration

This project has been configured to send tracing data to OpenTelemetry-compatible backends such as Jaeger.

## Features

- Distributed tracing with spans across the entire request lifecycle
- Automatic propagation of trace context
- Support for both Jaeger and OTLP protocol
- Detailed span attributes for request/response details
- Error tracking and status codes

## Configuration

You can configure the OpenTelemetry exporter using environment variables:

- `JAEGER_ENDPOINT`: The URL of the Jaeger collector (default: http://localhost:14268/api/traces)
- `OTLP_ENDPOINT`: The URL of an OpenTelemetry protocol (OTLP) collector

If neither of these is set, the application will default to using Jaeger on localhost.

## Running with Jaeger

The easiest way to get started is to run Jaeger using Docker:

```bash
docker run -d --name jaeger \
  -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 14250:14250 \
  -p 14268:14268 \
  -p 14269:14269 \
  -p 9411:9411 \
  jaegertracing/all-in-one:latest
```

Then, start the application with the Jaeger endpoint:

```bash
JAEGER_ENDPOINT=http://localhost:14268/api/traces cargo run
```

## Viewing Traces

Once your application is running and generating spans:

1. Open Jaeger UI in your browser at http://localhost:16686
2. Select "saasexpress" from the Service dropdown
3. Click "Find Traces" to view the available traces

## Using OTLP Instead of Jaeger

If you prefer to use the OpenTelemetry Protocol (OTLP):

```bash
OTLP_ENDPOINT=http://localhost:4318/v1/traces cargo run
```

## Available Spans

The application creates the following spans:

- `bootstrap`: Application startup
- `build_graph`: Graph construction
- `process_nodes`: Node processing during graph construction
- `process_edges`: Edge processing during graph construction
- `http_server_start`: HTTP server initialization
- `http_request`: Complete HTTP request handling
- `parse_request`: Request body parsing
- `process_request`: Forwarding request to operator
- `wait_response`: Waiting for operator response
- `format_response`: Formatting the HTTP response

Each span includes relevant attributes such as request IDs, paths, methods, and status codes.

## Custom Instrumentation

To add custom instrumentation to your code:

```rust
use tracing::{info_span, Span};

// Create and enter a span
let span = info_span!("my_operation", attribute = "value");
let _guard = span.enter();

// Record additional attributes
Span::current().record("key", "value");

// Log events within a span
tracing::info!("Something happened");
```