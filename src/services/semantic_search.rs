use diesel::{PgConnection, RunQueryDsl, sql_query};

use crate::{
    models::semantic_search_result::SemanticSearchResult, services::embedders::text::TextEmbedder,
};

pub fn search(
    conn: &mut PgConnection,
    query: &str,
    threshold: Option<f32>,
    limit: Option<usize>,
) -> Vec<SemanticSearchResult> {
    let text_embedder = TextEmbedder::new().unwrap();
    let embedding = text_embedder.embed(query).unwrap();
    let embedding_str = format!(
        "[{}]",
        embedding
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    let results = sql_query("SELECT * FROM similarity_search($1::vector, $2, $3)")
        .bind::<diesel::sql_types::Text, _>(embedding_str)
        .bind::<diesel::sql_types::Float, _>(threshold.unwrap_or(0.0))
        .bind::<diesel::sql_types::Integer, _>(limit.unwrap_or(10) as i32)
        .get_results::<SemanticSearchResult>(conn)
        .unwrap();

    results
}
