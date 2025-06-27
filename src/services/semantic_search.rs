use crate::{
    models::semantic_search_result::SemanticSearchResult, photo_repository::PhotoRepository,
    services::embedders::text::TextEmbedder,
};
use anyhow::{Context, Result};

pub struct SemanticSearchService<R: PhotoRepository, E: TextEmbedder> {
    photo_repository: R,
    text_embedder: E,
}

impl<R: PhotoRepository, E: TextEmbedder> SemanticSearchService<R, E> {
    pub fn new(photo_repository: R, text_embedder: E) -> Self {
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
    ) -> Result<Vec<SemanticSearchResult>> {
        let embedding = self
            .text_embedder
            .embed(query)
            .context("Failed to create text embedding")?;

        self.photo_repository
            .find_by_text(embedding, threshold, limit)
            .context("Failed to search by text")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        photo_repository::MockPhotoRepository, services::embedders::text::MockTextEmbedder,
    };

    use super::*;
    use anyhow::anyhow;
    use mockall::predicate::eq;

    #[test]
    fn test_should_return_error_when_embedder_fails() {
        let mut text_embedder = MockTextEmbedder::new();
        text_embedder
            .expect_embed()
            .with(eq("test"))
            .returning(|_| Err(anyhow!("Embedding error")));

        let photo_repository = MockPhotoRepository::new();
        let mut service = SemanticSearchService::new(photo_repository, text_embedder);
        let result = service.search("test", None, None);

        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to create text embedding"
        );
    }
}
