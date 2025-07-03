// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use postgis_diesel::sql_types::*;

    cities (geonameid) {
        geonameid -> Int4,
        name -> Text,
        asciiname -> Nullable<Text>,
        alternatenames -> Nullable<Text>,
        latitude -> Float4,
        longitude -> Float4,
        feature_class -> Nullable<Text>,
        feature_code -> Nullable<Text>,
        country_code -> Nullable<Text>,
        cc2 -> Nullable<Text>,
        admin1_code -> Nullable<Text>,
        admin2_code -> Nullable<Text>,
        admin3_code -> Nullable<Text>,
        admin4_code -> Nullable<Text>,
        population -> Nullable<Int4>,
        elevation -> Nullable<Int4>,
        dem -> Nullable<Int4>,
        timezone -> Nullable<Text>,
        modification_date -> Nullable<Text>,
        geom -> Nullable<Geometry>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use postgis_diesel::sql_types::*;

    countries (gid) {
        gid -> Int4,
        #[max_length = 29]
        name -> Nullable<Varchar>,
        #[max_length = 36]
        name_long -> Nullable<Varchar>,
        #[max_length = 52]
        formal_en -> Nullable<Varchar>,
        #[max_length = 35]
        formal_fr -> Nullable<Varchar>,
        pop_est -> Nullable<Float8>,
        pop_rank -> Nullable<Int2>,
        pop_year -> Nullable<Int2>,
        gdp_md -> Nullable<Int4>,
        gdp_year -> Nullable<Int2>,
        #[max_length = 26]
        economy -> Nullable<Varchar>,
        #[max_length = 23]
        income_grp -> Nullable<Varchar>,
        #[max_length = 5]
        iso_a2 -> Nullable<Varchar>,
        #[max_length = 3]
        iso_a3 -> Nullable<Varchar>,
        #[max_length = 3]
        iso_n3 -> Nullable<Varchar>,
        #[max_length = 3]
        adm0_a3_id -> Nullable<Varchar>,
        #[max_length = 23]
        continent -> Nullable<Varchar>,
        #[max_length = 10]
        region_un -> Nullable<Varchar>,
        #[max_length = 25]
        subregion -> Nullable<Varchar>,
        #[max_length = 26]
        region_wb -> Nullable<Varchar>,
        name_len -> Nullable<Int2>,
        long_len -> Nullable<Int2>,
        abbrev_len -> Nullable<Int2>,
        #[max_length = 8]
        wikidataid -> Nullable<Varchar>,
        geom -> Nullable<Geometry>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use postgis_diesel::sql_types::*;

    faces (id) {
        id -> Int4,
        photo_id -> Int4,
        bbox_x -> Int4,
        bbox_y -> Int4,
        bbox_width -> Int4,
        bbox_height -> Int4,
        confidence -> Float4,
        #[max_length = 10]
        gender -> Nullable<Varchar>,
        embedding -> Nullable<Vector>,
        person_id -> Nullable<Int4>,
        recognition_confidence -> Nullable<Float4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use postgis_diesel::sql_types::*;

    people (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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
        face_detection_completed -> Bool,
        country_id -> Nullable<Int4>,
        city_id -> Nullable<Int4>,
        indexed_at -> Timestamptz,
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

diesel::joinable!(faces -> people (person_id));
diesel::joinable!(faces -> photos (photo_id));
diesel::joinable!(photos -> cities (city_id));
diesel::joinable!(photos -> countries (country_id));

diesel::allow_tables_to_appear_in_same_query!(
    cities,
    countries,
    faces,
    people,
    photos,
    spatial_ref_sys,
);
