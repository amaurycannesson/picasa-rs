use diesel::{PgConnection, RunQueryDsl, sql_query};

use crate::models::geospatial_search_result::GeospatialSearchResult;

pub fn search(conn: &mut PgConnection, country_query: &str) -> Vec<GeospatialSearchResult> {
    let results = sql_query("SELECT * FROM find_photos_by_country($1)")
        .bind::<diesel::sql_types::Text, _>(country_query)
        .get_results::<GeospatialSearchResult>(conn)
        .unwrap();

    results
}
