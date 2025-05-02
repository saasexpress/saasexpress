use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::{OpenApi, ToSchema};

pub mod activity_handler;
pub mod service_handler;
pub mod tenant_handler;

use activity_handler::*;
use service_handler::*;
use tenant_handler::*;
use crate::models::{TenantDTO, ActivityDTO, ServiceDTO, DagVariantDTO};

#[derive(Clone)]
pub struct AppState {
    // This could hold shared application state, like DB connections
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ApiError {
    pub error: String,
    pub status: u16,
}

impl ApiError {
    pub fn not_found(message: String) -> Self {
        Self {
            error: message,
            status: 404,
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            error: message,
            status: 400,
        }
    }

    pub fn internal(message: String) -> Self {
        Self {
            error: message,
            status: 500,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(self);
        (status, body).into_response()
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_tenants,
        get_tenant,
        create_tenant,
        update_tenant,
        delete_tenant,
        get_activities,
        create_activity,
        delete_activity,
        get_services,
        get_service,
        create_service,
        update_service,
        delete_service
    ),
    components(
        schemas(ApiError, TenantDTO, ActivityDTO, ServiceDTO, DagVariantDTO, PaginationParams)
    ),
    tags(
        (name = "tenants", description = "Tenant management API"),
        (name = "activity", description = "Activity logging API"),
        (name = "services", description = "Service management API")
    )
)]
pub struct ApiDoc;

pub fn create_router() -> Router<Arc<AppState>> {
    let state = Arc::new(AppState {});

    Router::new()
        .route("/api/tenants", get(get_tenants))
        .route("/api/tenants", post(create_tenant))
        .route("/api/tenants/:id", get(get_tenant))
        .route("/api/tenants/:id", put(update_tenant))
        .route("/api/tenants/:id", delete(delete_tenant))
        .route("/api/activity", get(get_activities))
        .route("/api/activity", post(create_activity))
        .route("/api/activity/:id", delete(delete_activity))
        .route("/api/services", get(get_services))
        .route("/api/services", post(create_service))
        .route("/api/services/:id", get(get_service))
        .route("/api/services/:id", put(update_service))
        .route("/api/services/:id", delete(delete_service))
        .with_state(state)
}
