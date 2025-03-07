FROM rust:1.78-slim as builder

WORKDIR /usr/src/admin-api-rs
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev libsqlite3-dev

# Build the application
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary and static files
COPY --from=builder /usr/src/admin-api-rs/target/release/admin-api-rs .
COPY --from=builder /usr/src/admin-api-rs/ui ./ui

# Set the environment variables
ENV RUST_LOG=info

# Expose the port
EXPOSE 8081

# Run the application
CMD ["./admin-api-rs"]