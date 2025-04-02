FROM rust:latest as builder

WORKDIR /usr/src/rust-microservice

# Create empty project for caching dependencies
RUN cargo new --bin tenant-gateway
WORKDIR /usr/src/tenant-gateway/tenant-gateway

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock* ./

# Build dependencies only (this layer is cached if dependencies don't change)
# RUN cargo build --release
# RUN rm src/*.rs

# Copy the actual source code
COPY src ./src
COPY bootstrap ./bootstrap

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
COPY --from=builder /usr/src/tenant-gateway/tenant-gateway/target/release/tenant-gateway /app/

# Set non-root user for security
RUN useradd -m appuser
USER appuser

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=3000
ENV ENVIRONMENT=production

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["/app/tenant-gateway"]