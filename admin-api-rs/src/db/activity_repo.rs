use diesel::prelude::*;
use diesel::result::Error;

use crate::db::get_conn;
use crate::models::{Activity, NewActivity};
use crate::schema::activities;

pub fn get_all_activities(page: i64, records_per_page: i64) -> Result<Vec<Activity>, Error> {
    use crate::schema::activities::dsl::*;

    let conn = &mut get_conn();
    activities
        .filter(deleted_at.is_null())
        .order(activity_at.desc())
        .then_order_by(id.desc())
        .offset(page * records_per_page)
        .limit(records_per_page)
        .load::<Activity>(conn)
}

pub fn count_activities() -> Result<i64, Error> {
    use crate::schema::activities::dsl::*;
    use diesel::dsl::count;

    let conn = &mut get_conn();
    activities
        .filter(deleted_at.is_null())
        .select(count(id))
        .first(conn)
}

pub fn create_activity(new_activity: NewActivity) -> Result<Activity, Error> {
    let conn = &mut get_conn();
    
    diesel::insert_into(activities::table)
        .values(&new_activity)
        .execute(conn)?;

    activities::table
        .order(activities::created_at.desc())
        .first(conn)
}

pub fn delete_activity(activity_id: i32) -> Result<usize, Error> {
    use crate::schema::activities::dsl::*;

    let conn = &mut get_conn();
    
    diesel::delete(activities.filter(id.eq(activity_id)))
        .execute(conn)
}