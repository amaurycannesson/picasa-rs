use diesel::prelude::*;

use crate::models::photo::Photo;

#[derive(Debug, QueryableByName)]
pub struct SemanticSearchResult {
    #[diesel(embed)]
    pub photo: Photo,
    #[diesel(sql_type = diesel::sql_types::Float)]
    pub similarity: f32,
}
