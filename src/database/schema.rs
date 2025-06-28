// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use postgis_diesel::sql_types::*;

    photos (id) {
        id -> Int4,
        path -> Text,
        file_name -> Text,
        file_size -> Int8,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
        indexed_at -> Timestamptz,
        hash -> Nullable<Text>,
        camera_make -> Nullable<Text>,
        camera_model -> Nullable<Text>,
        lens_model -> Nullable<Text>,
        orientation -> Nullable<Int4>,
        date_taken_local -> Nullable<Timestamp>,
        date_taken_utc -> Nullable<Timestamptz>,
        gps_location -> Nullable<Geometry>,
        image_width -> Nullable<Int4>,
        image_height -> Nullable<Int4>,
        embedding -> Nullable<Vector>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use postgis_diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    photos,
    spatial_ref_sys,
);
