use diesel::prelude::*;
use diesel::result::Error;
use std::collections::HashMap;
use tracing::error;

use crate::db::{generate_random_id, get_conn};
use crate::models::{DagVariant, NewDagVariant, NewService, Service, UpdateService};
use crate::schema::{dag_variants, services};

pub fn get_all_services(page: i64, records_per_page: i64) -> Result<Vec<Service>, Error> {
    use crate::schema::services::dsl::*;

    let conn = &mut get_conn();
    services
        .filter(deleted_at.is_null())
        .offset(page * records_per_page)
        .limit(records_per_page)
        .load::<Service>(conn)
}

pub fn count_services() -> Result<i64, Error> {
    use crate::schema::services::dsl::*;
    use diesel::dsl::count;

    let conn = &mut get_conn();
    services
        .filter(deleted_at.is_null())
        .select(count(id))
        .first(conn)
}

pub fn get_service_by_id(service_id: &str) -> Result<Service, Error> {
    use crate::schema::services::dsl::*;

    let conn = &mut get_conn();
    services
        .filter(id.eq(service_id))
        .filter(deleted_at.is_null())
        .first::<Service>(conn)
}

pub fn get_variants_for_service(a_service_id: &str) -> Result<Vec<DagVariant>, Error> {
    use crate::schema::dag_variants::dsl::*;

    let conn = &mut get_conn();
    dag_variants
        .filter(service_id.eq(a_service_id))
        .filter(deleted_at.is_null())
        .load::<DagVariant>(conn)
}

pub fn get_variants_with_names(a_service_id: &str) -> Result<Vec<(String, DagVariant)>, Error> {
    let variants = get_variants_for_service(a_service_id)?;

    let result = variants.into_iter().map(|v| (v.name.clone(), v)).collect();
    Ok(result)
}

pub fn create_service(
    new_service: NewService,
    variants: HashMap<String, String>,
) -> Result<Service, Error> {
    let conn = &mut get_conn();

    let service_id = match new_service.id {
        Some(sid) => sid,
        None => generate_random_id(15),
    };

    let service_with_id = NewService {
        id: Some(service_id.clone()),
        display_name: new_service.display_name,
        service_url: new_service.service_url,
    };

    conn.transaction(|tx| {
        diesel::insert_into(services::table)
            .values(&service_with_id)
            .execute(tx)?;

        for (name, dag_json) in variants {
            let new_variant = NewDagVariant {
                name,
                dag: dag_json,
                service_id: service_id.clone(),
            };

            diesel::insert_into(dag_variants::table)
                .values(&new_variant)
                .execute(tx)?;
        }

        services::table
            .filter(services::id.eq(&service_id))
            .first::<Service>(tx)
    })
}

pub fn update_service(
    service_id: &str,
    service_update: UpdateService,
    variants: Option<HashMap<String, String>>,
) -> Result<Service, Error> {
    let conn = &mut get_conn();

    conn.transaction(|tx| {
        // Update service
        diesel::update(services::table.filter(services::id.eq(service_id)))
            .set(&service_update)
            .execute(tx)?;

        // If variants were provided, update them
        if let Some(variant_updates) = variants {
            // Delete existing variants
            diesel::delete(dag_variants::table.filter(dag_variants::service_id.eq(service_id)))
                .execute(tx)?;

            // Add new variants
            for (name, dag_json) in variant_updates {
                let new_variant = NewDagVariant {
                    name,
                    dag: dag_json,
                    service_id: service_id.to_string(),
                };

                diesel::insert_into(dag_variants::table)
                    .values(&new_variant)
                    .execute(tx)?;
            }
        }

        services::table
            .filter(services::id.eq(service_id))
            .first::<Service>(tx)
    })
}

pub fn delete_service(service_id: &str) -> Result<usize, Error> {
    use crate::schema::services::dsl::*;

    let conn = &mut get_conn();

    diesel::delete(services.filter(id.eq(service_id))).execute(conn)
}
