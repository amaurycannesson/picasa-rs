use anyhow::{Context, Result};
use pgvector::Vector;
use std::time::Instant;

use crate::{
    models::photo::PhotoEmbedding, photo_repository::PhotoRepository,
    services::embedders::image::ImageEmbedder, utils::progress_reporter::ProgressReporter,
};

pub struct PhotoEmbedderService<R: PhotoRepository, E: ImageEmbedder, P: ProgressReporter> {
    photo_repository: R,
    image_embedder: E,
    progress_reporter: P,
}

impl<R: PhotoRepository, E: ImageEmbedder, P: ProgressReporter> PhotoEmbedderService<R, E, P> {
    pub fn new(photo_repository: R, image_embedder: E, progress_reporter: P) -> Self {
        Self {
            photo_repository,
            image_embedder,
            progress_reporter,
        }
    }

    /// Compute embeddings
    pub fn embed(&mut self) -> Result<usize> {
        let start = Instant::now();
        let per_page = 20i64;
        let mut page = 1i64;
        let mut total_processed = 0usize;

        loop {
            // Get the next batch of photos without embeddings
            let paginated_paths = self
                .photo_repository
                .list_paths_without_embedding(page, per_page)
                .context("Failed to fetch photos without embeddings")?;

            // If no more photos to process, break
            if paginated_paths.paths.is_empty() {
                break;
            }

            self.progress_reporter.set_message(format!(
                "Processing batch {} ({} photos remaining)",
                page,
                paginated_paths.total - (page - 1) * per_page
            ));

            // Compute embeddings for this batch
            let embeddings_data = self
                .image_embedder
                .embed(&paginated_paths.paths)
                .context("Failed to compute embeddings")?;

            // Create PhotoEmbedding structs
            let photo_embeddings: Vec<PhotoEmbedding> = paginated_paths
                .paths
                .iter()
                .zip(embeddings_data.iter())
                .map(|(path, embedding_vec)| PhotoEmbedding {
                    path: path.clone(),
                    embedding: Some(Vector::from(embedding_vec.clone())),
                })
                .collect();

            // Update embeddings in batch
            let updated_count = self
                .photo_repository
                .update_embeddings(photo_embeddings)
                .context("Failed to update embeddings in database")?;

            total_processed += updated_count;
            self.progress_reporter.set_message(format!(
                "Processed {} photos (batch {}, {} total)",
                updated_count, page, total_processed
            ));

            page += 1;
        }

        let duration = start.elapsed();
        self.progress_reporter.finish_with_message(format!(
            "✓ Processed embeddings for {} photos in {:.2?}",
            total_processed, duration
        ));

        Ok(total_processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::photo::PaginatedPaths, photo_repository::MockPhotoRepository,
        services::embedders::image::MockImageEmbedder,
        utils::progress_reporter::NoOpProgressReporter,
    };
    use anyhow::anyhow;
    use mockall::predicate::*;

    #[test]
    fn test_process_embeddings_no_photos() {
        let mut photo_repository = MockPhotoRepository::new();
        photo_repository
            .expect_list_paths_without_embedding()
            .with(eq(1), eq(20))
            .times(1)
            .returning(|_, _| {
                Ok(PaginatedPaths {
                    paths: vec![],
                    total: 0,
                    page: 1,
                    per_page: 20,
                })
            });
        photo_repository.expect_update_embeddings().times(0);

        let mut image_embedder = MockImageEmbedder::new();
        image_embedder
            .expect_embed()
            .times(0)
            .returning(|_| Ok(vec![]));

        let mut photo_embedder_service =
            PhotoEmbedderService::new(photo_repository, image_embedder, NoOpProgressReporter);
        let result = photo_embedder_service.embed();

        assert!(result.is_ok());
    }

    #[test]
    fn test_should_return_error_when_repository_fails() {
        let mut photo_repository = MockPhotoRepository::new();

        photo_repository
            .expect_list_paths_without_embedding()
            .with(eq(1), eq(20))
            .times(1)
            .returning(|_, _| Err(anyhow!("Repository error")));

        let mut image_embedder = MockImageEmbedder::new();

        image_embedder
            .expect_embed()
            .times(0)
            .returning(|_| Ok(vec![]));

        let mut photo_embedder_service =
            PhotoEmbedderService::new(photo_repository, image_embedder, NoOpProgressReporter);
        let result = photo_embedder_service.embed();

        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to fetch photos without embeddings"
        );
    }

    #[test]
    fn test_should_return_error_when_update_embeddings_fails() {
        let mut photo_repository = MockPhotoRepository::new();

        photo_repository
            .expect_list_paths_without_embedding()
            .with(eq(1), eq(20))
            .times(1)
            .returning(|_, _| {
                Ok(PaginatedPaths {
                    paths: vec!["test.jpg".to_string()],
                    total: 1,
                    page: 1,
                    per_page: 20,
                })
            });

        photo_repository
            .expect_update_embeddings()
            .times(1)
            .returning(|_| Err(anyhow!("Failed to update embeddings in database")));

        let mut image_embedder = MockImageEmbedder::new();

        image_embedder
            .expect_embed()
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut photo_embedder_service =
            PhotoEmbedderService::new(photo_repository, image_embedder, NoOpProgressReporter);
        let result = photo_embedder_service.embed();

        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to update embeddings in database"
        );
    }
}
