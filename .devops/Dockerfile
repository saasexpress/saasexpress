FROM rust:latest AS builder

WORKDIR /usr/src/app

# Create empty project for caching dependencies
RUN cargo new --bin saasexpress
WORKDIR /usr/src/app/saasexpress

# Copy the dependent modules
COPY saasexpress-core ./saasexpress-core
COPY saasexpress-tenants ./saasexpress-tenants

RUN ls -la ./saasexpress-tenants/ui

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock* ./

# Build dependencies only (this layer is cached if dependencies don't change)
# RUN touch src/main.rs && cargo build --release
# RUN rm src/*.rs

# Copy the actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs && cargo build --release

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder
COPY --from=builder /usr/src/app/saasexpress/target/release/saasexpress /app/

# Set non-root user for security
RUN useradd -m appuser
USER appuser

# Run the application
ENTRYPOINT [ "/app/saasexpress" ]