mod api;
mod bootstrap;
mod db;
mod models;
mod schema;
mod static_handler;

use crate::static_handler::static_handler;
use axum::Router;
use serde_yaml::Value;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{ApiDoc, create_router};

pub struct TenantsService;

impl TenantsService {
    pub fn saasexpress_graphs() -> Vec<Value> {
        bootstrap::gather_files()
    }

    pub async fn start() {
        // Initialize database
        db::get_pool();

        // Create the API router
        let api_router = create_router();

        // Create the Swagger UI router
        let swagger_ui = SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi());

        // Build the application with SwaggerUI
        let app = Router::new()
            .nest("/", api_router)
            .merge(swagger_ui)
            .fallback(static_handler)
            .layer(TraceLayer::new_for_http());

        // Run the server
        let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
        tracing::info!("[Tenants] Listening on {}", addr);

        // Convert our Router to a standard service-based Router
        let app_service = app.with_state(std::sync::Arc::new(api::AppState {}));

        // Start the server
        if let Err(e) = axum::Server::bind(&addr)
            .serve(app_service.into_make_service())
            .await
        {
            eprintln!("server error: {}", e);
        }
    }
}
