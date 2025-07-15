use diesel::sql_types::*;
use postgis_diesel::sql_types::*;

use diesel::define_sql_function;

define_sql_function! {
    #[sql_name = "find_country_id_by_geom"]
    fn find_country_id_by_geom(geom_query: Geometry) -> Nullable<Integer>;
}

define_sql_function! {
    #[sql_name = "find_city_id_by_geom"]
    fn find_city_id_by_geom(geom_query: Geometry, radius: Numeric) -> Nullable<Integer>;
}

define_sql_function! {
    #[sql_name = "find_country_id_by_name"]
    fn find_country_id_by_name(name_query: Text) -> Nullable<Integer>;
}

define_sql_function! {
    #[sql_name = "find_city_id_by_name"]
    fn find_city_id_by_name(name_query: Text) -> Nullable<Integer>;
}

define_sql_function! { fn coalesce(x: Nullable<Text>, y: Text) -> Text; }
