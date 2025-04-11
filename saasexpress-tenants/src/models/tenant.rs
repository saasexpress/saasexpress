use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::tenants;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone, ToSchema)]
#[diesel(table_name = tenants)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tenant {
    pub id: String,
    pub display_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = tenants)]
pub struct NewTenant {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = tenants)]
pub struct UpdateTenant {
    pub display_name: Option<String>,
}

// API DTOs
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct TenantDTO {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

impl From<Tenant> for TenantDTO {
    fn from(tenant: Tenant) -> Self {
        Self {
            id: Some(tenant.id),
            display_name: tenant.display_name,
        }
    }
}

impl From<TenantDTO> for NewTenant {
    fn from(dto: TenantDTO) -> Self {
        Self {
            id: dto.id,
            display_name: dto.display_name,
        }
    }
}

impl From<TenantDTO> for UpdateTenant {
    fn from(dto: TenantDTO) -> Self {
        Self {
            display_name: dto.display_name,
        }
    }
}
