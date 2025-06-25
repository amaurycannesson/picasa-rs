use diesel::prelude::*;

#[derive(Debug, QueryableByName)]
pub struct GeospatialSearchResult {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub path: String,
}
