use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use std::sync::Arc;
use utoipa::{ToSchema, IntoParams};

use crate::db::activity_repo;
use crate::models::{ActivityDTO, NewActivity};
use crate::api::{AppState, ApiError};

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_records_per_page")]
    pub records_per_page: i64,
}

fn default_page() -> i64 {
    0
}

fn default_records_per_page() -> i64 {
    25
}

#[utoipa::path(
    get,
    path = "/activity",
    tag = "activity",
    params(
        PaginationParams
    ),
    responses(
        (status = 200, description = "List of activities retrieved successfully", body = [ActivityDTO]),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn get_activities(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Response, ApiError> {
    let activities = activity_repo::get_all_activities(params.page, params.records_per_page)
        .map_err(|e| match e {
            DieselError::NotFound => ApiError::not_found("No activities found".to_string()),
            _ => ApiError::internal(e.to_string()),
        })?;

    let total_records = activity_repo::count_activities()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let total_pages = ((total_records as f64) / (params.records_per_page as f64)).ceil() as i64;

    let activity_dtos: Vec<ActivityDTO> = activities.into_iter().map(ActivityDTO::from).collect();
    
    let response = axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("paging-total-records", total_records.to_string())
        .header("paging-total-pages", total_pages.to_string())
        .header("paging-current-page", params.page.to_string())
        .header("paging-page-size", params.records_per_page.to_string())
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::boxed(axum::body::Full::from(
            serde_json::to_string(&activity_dtos).unwrap()
        )))
        .unwrap();

    Ok(response)
}

#[utoipa::path(
    post,
    path = "/activity",
    tag = "activity",
    request_body = ActivityDTO,
    responses(
        (status = 201, description = "Activity created successfully", body = ActivityDTO),
        (status = 400, description = "Invalid activity data", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn create_activity(
    State(_state): State<Arc<AppState>>,
    Json(activity): Json<ActivityDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let new_activity = NewActivity::from(activity);
    
    let created_activity = activity_repo::create_activity(new_activity)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ActivityDTO::from(created_activity))))
}

#[utoipa::path(
    delete,
    path = "/activity/{id}",
    tag = "activity",
    params(
        ("id" = i32, Path, description = "Activity ID")
    ),
    responses(
        (status = 204, description = "Activity deleted successfully"),
        (status = 404, description = "Activity not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
pub async fn delete_activity(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let result = activity_repo::delete_activity(id)
        .map_err(|e| ApiError::internal(e.to_string()))?;

    if result == 0 {
        return Err(ApiError::not_found(format!("Activity with ID {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}