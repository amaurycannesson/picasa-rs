use diesel::prelude::*;

#[derive(Debug, QueryableByName)]
pub struct SemanticSearchResult {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub path: String,
    #[diesel(sql_type = diesel::sql_types::Float)]
    pub similarity: f32,
}
