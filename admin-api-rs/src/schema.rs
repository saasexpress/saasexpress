// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Integer,
        activity_at -> Timestamp,
        message -> Nullable<Text>,
        params -> Nullable<Text>,
        result -> Nullable<Text>,
        filter_ukey1 -> Nullable<Text>,
        filter_key1 -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    dag_variants (id) {
        id -> Integer,
        name -> Text,
        dag -> Text,
        service_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    services (id) {
        id -> Text,
        display_name -> Text,
        service_url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tenants (id) {
        id -> Text,
        display_name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(dag_variants -> services (service_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    dag_variants,
    services,
    tenants,
);