use crate::{
    models::semantic_search_result::SemanticSearchResult, photo_repository::PhotoRepository,
    services::embedders::text::TextEmbedder,
};

pub struct SemanticSearchService<R: PhotoRepository> {
    photo_repository: R,
    text_embedder: TextEmbedder,
}

impl<R: PhotoRepository> SemanticSearchService<R> {
    pub fn new(photo_repository: R, text_embedder: TextEmbedder) -> Self {
        Self {
            photo_repository,
            text_embedder,
        }
    }

    pub fn search(
        &mut self,
        query: &str,
        threshold: Option<f32>,
        limit: Option<usize>,
    ) -> Vec<SemanticSearchResult> {
        let embedding = self.text_embedder.embed(query).unwrap();

        let results = self
            .photo_repository
            .find_by_text(embedding, threshold, limit)
            .unwrap_or_else(|_| vec![]);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    #[test]
    fn test_should_return_error_when_embedder_fails() {
        // let mut conn = PgConnection::establish("postgres://user:password@localhost/db").unwrap();
        // let mut text_embedder = TextEmbedder::new().unwrap();
        // text_embedder
        //     .expect_embed()
        //     .with(eq("test"))
        //     .returning(|_| Err("Embedding error".into()));

        // let service = SemanticSearchService::new(&mut conn, text_embedder);
        // let result = service.search(&mut conn, "test", None, None);

        // assert!(
        //     result.is_empty(),
        //     "Expected no results due to embedding error"
        // );
    }
}
