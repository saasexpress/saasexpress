mod api;
mod db;
mod models;
mod schema;

use axum::{
    Router, 
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    body::{self, Full}
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use rust_embed::RustEmbed;

use crate::api::{create_router, ApiDoc};

#[derive(RustEmbed)]
#[folder = "ui/"]
struct StaticAssets;

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    
    match StaticAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body::boxed(Full::from(content.data)))
                .unwrap()
        },
        None => {
            // If the path doesn't exist, try to serve index.html (for SPA routing)
            if let Some(content) = StaticAssets::get("index.html") {
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(body::boxed(Full::from(content.data)))
                    .unwrap()
            } else {
                // Truly not found
                (StatusCode::NOT_FOUND, "Not Found").into_response()
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "admin_api_rs=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    db::get_pool();

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the API router
    let api_router = create_router();
    
    // Create the Swagger UI router
    let swagger_ui = SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi());

    // Build the application with SwaggerUI
    let app = Router::new()
        .nest("/", api_router)
        .merge(swagger_ui)
        .fallback(static_handler)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("Listening on {}", addr);
    
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