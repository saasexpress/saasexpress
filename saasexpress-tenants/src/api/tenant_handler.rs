use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::result::Error as DieselError;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::db::tenant_repo;
use crate::models::{TenantDTO, NewTenant, UpdateTenant};
use crate::api::{AppState, ApiError};

#[utoipa::path(
    get,
    path = "/tenants",
    tag = "tenants",
    responses(
        (status = 200, description = "List of tenants retrieved successfully", body = [TenantDTO]),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn get_tenants(
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let tenants = tenant_repo::get_all_tenants()
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found("No tenants found".to_string()),
            _ => ApiError::internal(e.to_string()),
        })?;

    let tenant_dtos: Vec<TenantDTO> = tenants.into_iter().map(TenantDTO::from).collect();
    Ok((StatusCode::OK, Json(tenant_dtos)))
}

#[utoipa::path(
    get,
    path = "/tenants/{id}",
    tag = "tenants",
    params(
        ("id" = String, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "Tenant retrieved successfully", body = TenantDTO),
        (status = 404, description = "Tenant not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn get_tenant(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let tenant = tenant_repo::get_tenant_by_id(&id)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found(format!("Tenant with ID {} not found", id)),
            _ => ApiError::internal(e.to_string()),
        })?;

    Ok((StatusCode::OK, Json(TenantDTO::from(tenant))))
}

#[utoipa::path(
    post,
    path = "/tenants",
    tag = "tenants",
    request_body = TenantDTO,
    responses(
        (status = 201, description = "Tenant created successfully", body = TenantDTO),
        (status = 400, description = "Invalid tenant data", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn create_tenant(
    State(_state): State<Arc<AppState>>,
    Json(tenant): Json<TenantDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let new_tenant = NewTenant::from(tenant);
    
    let created_tenant = tenant_repo::create_tenant(new_tenant)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(TenantDTO::from(created_tenant))))
}

#[utoipa::path(
    put,
    path = "/tenants/{id}",
    tag = "tenants",
    params(
        ("id" = String, Path, description = "Tenant ID")
    ),
    request_body = TenantDTO,
    responses(
        (status = 200, description = "Tenant updated successfully", body = TenantDTO),
        (status = 404, description = "Tenant not found", body = ApiError),
        (status = 400, description = "Invalid tenant data", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn update_tenant(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(tenant): Json<TenantDTO>,
) -> Result<impl IntoResponse, ApiError> {
    // Verify tenant exists
    tenant_repo::get_tenant_by_id(&id)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found(format!("Tenant with ID {} not found", id)),
            _ => ApiError::internal(e.to_string()),
        })?;

    let update_tenant = UpdateTenant::from(tenant);
    
    let updated_tenant = tenant_repo::update_tenant(&id, update_tenant)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(TenantDTO::from(updated_tenant))))
}

#[utoipa::path(
    delete,
    path = "/tenants/{id}",
    tag = "tenants",
    params(
        ("id" = String, Path, description = "Tenant ID")
    ),
    responses(
        (status = 204, description = "Tenant deleted successfully"),
        (status = 404, description = "Tenant not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn delete_tenant(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // Verify tenant exists
    tenant_repo::get_tenant_by_id(&id)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found(format!("Tenant with ID {} not found", id)),
            _ => ApiError::internal(e.to_string()),
        })?;

    tenant_repo::delete_tenant(&id)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}