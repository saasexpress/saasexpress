use diesel::prelude::*;
use diesel::result::Error;

use crate::db::{get_conn, generate_random_id};
use crate::models::{Tenant, NewTenant, UpdateTenant};
use crate::schema::tenants;

pub fn get_all_tenants() -> Result<Vec<Tenant>, Error> {
    use crate::schema::tenants::dsl::*;

    let conn = &mut get_conn();
    tenants.filter(deleted_at.is_null())
        .load::<Tenant>(conn)
}

pub fn get_tenant_by_id(tenant_id: &str) -> Result<Tenant, Error> {
    use crate::schema::tenants::dsl::*;

    let conn = &mut get_conn();
    tenants.filter(id.eq(tenant_id))
        .filter(deleted_at.is_null())
        .first::<Tenant>(conn)
}

pub fn create_tenant(new_tenant: NewTenant) -> Result<Tenant, Error> {
    use crate::schema::tenants::dsl::*;

    let conn = &mut get_conn();
    
    let tenant_id = match new_tenant.id {
        Some(tid) => tid,
        None => generate_random_id(15),
    };

    let tenant_data = (
        id.eq(tenant_id),
        display_name.eq(new_tenant.display_name),
    );

    diesel::insert_into(tenants)
        .values(tenant_data)
        .execute(conn)?;

    tenants.order(created_at.desc())
        .first(conn)
}

pub fn update_tenant(tenant_id: &str, tenant_update: UpdateTenant) -> Result<Tenant, Error> {
    use crate::schema::tenants::dsl::*;

    let conn = &mut get_conn();
    
    diesel::update(tenants.filter(id.eq(tenant_id)))
        .set(&tenant_update)
        .execute(conn)?;

    tenants.filter(id.eq(tenant_id))
        .first(conn)
}

pub fn delete_tenant(tenant_id: &str) -> Result<usize, Error> {
    use crate::schema::tenants::dsl::*;

    let conn = &mut get_conn();
    
    diesel::delete(tenants.filter(id.eq(tenant_id)))
        .execute(conn)
}