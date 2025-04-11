use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::schema::{services, dag_variants};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone, ToSchema)]
#[diesel(table_name = services)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Service {
    pub id: String,
    pub display_name: String,
    pub service_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone, ToSchema, Associations)]
#[diesel(table_name = dag_variants)]
#[diesel(belongs_to(Service))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DagVariant {
    pub id: i32,
    pub name: String,
    pub dag: String, // JSON string
    pub service_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = services)]
pub struct NewService {
    pub id: Option<String>,
    pub display_name: String,
    pub service_url: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = dag_variants)]
pub struct NewDagVariant {
    pub name: String,
    pub dag: String, // JSON string
    pub service_id: String,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = services)]
pub struct UpdateService {
    pub display_name: Option<String>,
    pub service_url: Option<String>,
}

// API DTOs
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct DagVariantDTO {
    pub dag: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct ServiceDTO {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub service_url: Option<String>,
    pub variants: Option<HashMap<String, DagVariantDTO>>,
}

impl ServiceDTO {
    pub fn with_variants(mut self, variants: Vec<(String, DagVariant)>) -> Self {
        let variants_map = variants
            .into_iter()
            .map(|(name, variant)| {
                let dag: Value = serde_json::from_str(&variant.dag).unwrap_or_default();
                (name, DagVariantDTO { dag: Some(dag) })
            })
            .collect();
        
        self.variants = Some(variants_map);
        self
    }
}

impl From<Service> for ServiceDTO {
    fn from(service: Service) -> Self {
        Self {
            id: Some(service.id),
            display_name: Some(service.display_name),
            service_url: Some(service.service_url),
            variants: None,
        }
    }
}