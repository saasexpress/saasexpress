mod api;
mod db;
mod models;
mod schema;

use saasexpress_tenants::TenantsService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "saasexpress_tenants=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Start the tenants service
    tracing::info!("Starting tenants service...");
    TenantsService::start().await;
}
