use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::error;
use utoipa::ToSchema;

use crate::schema::activities;

pub fn naivedatetime_to_string(dt: &NaiveDateTime) -> String {
    dt.and_local_timezone(Utc).unwrap().to_rfc3339()
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone, ToSchema)]
#[diesel(table_name = activities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Activity {
    pub id: i32,
    pub activity_at: NaiveDateTime,
    pub message: Option<String>,
    pub params: Option<String>, // JSON string
    pub result: Option<String>,
    pub filter_ukey1: Option<String>,
    pub filter_key1: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = activities)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewActivity {
    pub activity_at: Option<NaiveDateTime>,
    pub message: Option<String>,
    pub params: Option<String>, // JSON string
    pub result: Option<String>,
    pub filter_ukey1: Option<String>,
    pub filter_key1: Option<String>,
}

// API DTOs
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ActivityDTO {
    pub id: Option<i32>,
    pub activity_at: Option<String>,
    pub message: Option<String>,
    pub params: Option<HashMap<String, String>>,
    pub result: Option<String>,
}

impl ActivityDTO {
    pub fn params_to_json(&self) -> Option<String> {
        self.params
            .as_ref()
            .map(|p| serde_json::to_string(p).unwrap_or_default())
    }
}

impl From<Activity> for ActivityDTO {
    fn from(activity: Activity) -> Self {
        let params: Option<HashMap<String, String>> =
            activity.params.and_then(|p| serde_json::from_str(&p).ok());

        Self {
            id: Some(activity.id),
            activity_at: Some(naivedatetime_to_string(&activity.activity_at)),
            message: activity.message,
            params,
            result: activity.result,
        }
    }
}

impl From<ActivityDTO> for NewActivity {
    fn from(dto: ActivityDTO) -> Self {
        // Clone activity_at to avoid the partial move during and_then
        let activity_at_clone = dto.activity_at.clone();
        let activity_at = activity_at_clone
            .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").ok())
            .unwrap_or_else(|| chrono::Utc::now().naive_utc());

        // Clone the message to avoid partial move
        let message = dto.message.clone();
        let params_json = dto.params_to_json();

        Self {
            activity_at: Some(activity_at),
            message,
            params: params_json,
            result: dto.result,
            filter_ukey1: None,
            filter_key1: None,
        }
    }
}
