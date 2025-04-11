use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::db::service_repo;
use crate::models::{ServiceDTO, NewService, UpdateService};
use crate::api::{AppState, ApiError};
use crate::api::activity_handler::PaginationParams;

#[utoipa::path(
    get,
    path = "/services",
    tag = "services",
    params(
        PaginationParams
    ),
    responses(
        (status = 200, description = "List of services retrieved successfully", body = [ServiceDTO]),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn get_services(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Response, ApiError> {
    let services = service_repo::get_all_services(params.page, params.records_per_page)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found("No services found".to_string()),
            _ => ApiError::internal(e.to_string()),
        })?;

    let total_records = service_repo::count_services()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let total_pages = ((total_records as f64) / (params.records_per_page as f64)).ceil() as i64;

    let mut service_dtos = Vec::new();
    
    for service in services {
        let variants = service_repo::get_variants_with_names(&service.id)
            .map_err(|e| ApiError::internal(e.to_string()))?;
        
        let service_dto = ServiceDTO::from(service).with_variants(variants);
        service_dtos.push(service_dto);
    }
    
    let response = axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("paging-total-records", total_records.to_string())
        .header("paging-total-pages", total_pages.to_string())
        .header("paging-current-page", params.page.to_string())
        .header("paging-page-size", params.records_per_page.to_string())
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::boxed(axum::body::Full::from(
            serde_json::to_string(&service_dtos).unwrap()
        )))
        .unwrap();

    Ok(response)
}

#[utoipa::path(
    get,
    path = "/services/{id}",
    tag = "services",
    params(
        ("id" = String, Path, description = "Service ID")
    ),
    responses(
        (status = 200, description = "Service retrieved successfully", body = ServiceDTO),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn get_service(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let service = service_repo::get_service_by_id(&id)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found(format!("Service with ID {} not found", id)),
            _ => ApiError::internal(e.to_string()),
        })?;

    let variants = service_repo::get_variants_with_names(&id)
        .map_err(|e| ApiError::internal(e.to_string()))?;
    
    let service_dto = ServiceDTO::from(service).with_variants(variants);

    Ok((StatusCode::OK, Json(service_dto)))
}

#[utoipa::path(
    post,
    path = "/services",
    tag = "services",
    request_body = ServiceDTO,
    responses(
        (status = 201, description = "Service created successfully", body = ServiceDTO),
        (status = 400, description = "Invalid service data", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn create_service(
    State(_state): State<Arc<AppState>>,
    Json(service_dto): Json<ServiceDTO>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate required fields
    let display_name = service_dto.display_name
        .clone()
        .ok_or_else(|| ApiError::bad_request("Display name is required".to_string()))?;
    
    let service_url = service_dto.service_url
        .clone()
        .ok_or_else(|| ApiError::bad_request("Service URL is required".to_string()))?;
    
    let new_service = NewService {
        id: service_dto.id.clone(),
        display_name,
        service_url,
    };
    
    // Process variants
    let variants = match service_dto.variants {
        Some(variants_map) => {
            let mut result = HashMap::new();
            for (name, variant) in variants_map {
                if let Some(dag) = variant.dag {
                    let dag_json = serde_json::to_string(&dag)
                        .map_err(|e| ApiError::bad_request(format!("Invalid DAG JSON: {}", e)))?;
                    result.insert(name, dag_json);
                }
            }
            result
        },
        None => HashMap::new(),
    };
    
    let created_service = service_repo::create_service(new_service, variants)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    // Fetch variants for the response
    let variants = service_repo::get_variants_with_names(&created_service.id)
        .map_err(|e| ApiError::internal(e.to_string()))?;
    
    let service_dto = ServiceDTO::from(created_service).with_variants(variants);

    Ok((StatusCode::CREATED, Json(service_dto)))
}

#[utoipa::path(
    put,
    path = "/services/{id}",
    tag = "services",
    params(
        ("id" = String, Path, description = "Service ID")
    ),
    request_body = ServiceDTO,
    responses(
        (status = 200, description = "Service updated successfully", body = ServiceDTO),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 400, description = "Invalid service data", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn update_service(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(service_dto): Json<ServiceDTO>,
) -> Result<impl IntoResponse, ApiError> {
    // Verify service exists
    service_repo::get_service_by_id(&id)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found(format!("Service with ID {} not found", id)),
            _ => ApiError::internal(e.to_string()),
        })?;

    let update_service = UpdateService {
        display_name: service_dto.display_name,
        service_url: service_dto.service_url,
    };
    
    // Process variants if provided
    let variants = service_dto.variants.map(|variants_map| {
        let mut result = HashMap::new();
        for (name, variant) in variants_map {
            if let Some(dag) = variant.dag {
                let dag_json = serde_json::to_string(&dag)
                    .unwrap_or_else(|_| json!({}).to_string());
                result.insert(name, dag_json);
            }
        }
        result
    });
    
    let updated_service = service_repo::update_service(&id, update_service, variants)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    // Fetch variants for the response
    let variants = service_repo::get_variants_with_names(&updated_service.id)
        .map_err(|e| ApiError::internal(e.to_string()))?;
    
    let service_dto = ServiceDTO::from(updated_service).with_variants(variants);

    Ok((StatusCode::OK, Json(service_dto)))
}

#[utoipa::path(
    delete,
    path = "/services/{id}",
    tag = "services",
    params(
        ("id" = String, Path, description = "Service ID")
    ),
    responses(
        (status = 204, description = "Service deleted successfully"),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn delete_service(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // Verify service exists
    service_repo::get_service_by_id(&id)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found(format!("Service with ID {} not found", id)),
            _ => ApiError::internal(e.to_string()),
        })?;

    service_repo::delete_service(&id)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}