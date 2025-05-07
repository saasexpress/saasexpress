mod api;
mod bootstrap;
mod db;
mod models;
mod schema;
mod static_handler;

use crate::static_handler::static_handler;
use api::ApiError;
use axum::Router;
use db::service_repo;
use models::{NewService, ServiceDTO};
use serde_yaml::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::{debug, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{ApiDoc, create_router};

pub struct TenantsService;

impl TenantsService {
    pub fn saasexpress_graphs() -> Vec<(String, Value)> {
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
            .fallback(static_handler);

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

    pub fn load_services() -> Result<(), ApiError> {
        let mut new_services = HashMap::<String, NewServiceSpec>::new();

        let graphs = bootstrap::gather_files();
        for (service_id, graph) in graphs {
            let name = graph["name"].as_str().unwrap();
            debug!("Loading service {} - graph: {}", service_id, name);
            // Load the service into the system
            if new_services.contains_key(&service_id) {
                let service = new_services.get_mut(&service_id).unwrap();

                if service.add_variant(name.to_string(), graph).is_err() {
                    return Err(ApiError::bad_request(format!(
                        "Invalid DAG JSON for service {}",
                        service_id
                    )));
                }
            } else {
                let other_display_name = format!("Service {}", service_id);
                let other_service_url = "http://example.com".to_string();

                let mut new_service = NewServiceSpec {
                    service: NewService {
                        id: None,
                        display_name: other_display_name.clone(),
                        service_url: other_service_url.clone(),
                    },
                    variants: HashMap::new(),
                };
                if new_service.add_variant(name.to_string(), graph).is_err() {
                    return Err(ApiError::bad_request(format!(
                        "Invalid DAG JSON for service {}",
                        service_id
                    )));
                }
                new_services.insert(service_id.clone(), new_service);
            }
        }

        for (_service_id, new_service) in new_services {
            let service = new_service.service;
            let variants = new_service.variants;

            // Create the service in the database
            let created_service = service_repo::create_service(service, variants)
                .map_err(|e| ApiError::internal(e.to_string()))?;

            // Fetch variants for the response
            let variants = service_repo::get_variants_with_names(&created_service.id)
                .map_err(|e| ApiError::internal(e.to_string()))?;

            let service_dto = ServiceDTO::from(created_service).with_variants(variants);

            info!("Service created: {:?}", service_dto);
        }

        Ok(())
    }
}

struct NewServiceSpec {
    service: NewService,
    variants: HashMap<String, String>,
}

impl NewServiceSpec {
    fn add_variant(&mut self, name: String, dag: serde_yaml::Value) -> Result<(), ApiError> {
        let dag_json = serde_json::to_string(&dag)
            .map_err(|e| ApiError::bad_request(format!("Invalid DAG JSON: {}", e)))?;
        self.variants.insert(name, dag_json);
        Ok(())
    }
}
