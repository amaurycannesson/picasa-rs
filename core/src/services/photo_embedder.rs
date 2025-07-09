use std::time::Instant;

use anyhow::{Context, Result};
use pgvector::Vector;

use crate::{
    models::{PaginationFilter, UpdatedPhoto},
    repositories::{PhotoFindPathFilters, PhotoRepository},
    services::embedders::image::ImageEmbedder,
    utils::progress_reporter::ProgressReporter,
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

    /// Computes embeddings for photos that don't have them yet.
    pub fn embed(&mut self) -> Result<usize> {
        let start = Instant::now();
        let mut total_processed = 0usize;

        loop {
            // Get the next batch of photos without embeddings
            let paginated_paths = self
                .photo_repository
                .find_path(
                    PaginationFilter {
                        page: 1,
                        per_page: 20,
                    },
                    PhotoFindPathFilters {
                        has_embedding: Some(false),
                        ..Default::default()
                    },
                )
                .context("Failed to fetch photos without embeddings")?;

            // If no more photos to process, break
            if paginated_paths.items.is_empty() {
                break;
            }

            self.progress_reporter.set_message(format!(
                "Processing photos: {} total, {} remaining",
                total_processed, paginated_paths.total
            ));

            // Compute embeddings for this batch
            let embeddings_data = self
                .image_embedder
                .embed(
                    &paginated_paths
                        .items
                        .iter()
                        .map(|f| f.path.clone())
                        .collect(),
                )
                .context("Failed to compute embeddings")?;

            for (photo, embedding) in paginated_paths.items.iter().zip(embeddings_data.iter()) {
                self.photo_repository
                    .update_one(
                        photo.id,
                        UpdatedPhoto {
                            embedding: Some(Some(Vector::from(embedding.clone()))),
                            ..Default::default()
                        },
                    )
                    .context("Failed to update embeddings in database")?;
            }

            total_processed += paginated_paths.items.len();
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
        models::{PaginatedPhotoPaths, PhotoPath},
        repositories::photo::repository::MockPhotoRepository,
        services::embedders::image::MockImageEmbedder,
        utils::progress_reporter::NoOpProgressReporter,
    };
    use anyhow::anyhow;

    #[test]
    fn test_process_embeddings_no_photos() {
        let mut photo_repository = MockPhotoRepository::new();
        photo_repository
            .expect_find_path()
            .withf(|p: &PaginationFilter, f: &PhotoFindPathFilters| {
                p.page == 1
                    && p.per_page == 20
                    && f.has_embedding == Some(false)
                    && f.has_face_detection_completed == None
            })
            .times(1)
            .returning(|_, __| {
                Ok(PaginatedPhotoPaths {
                    items: vec![],
                    total: 0,
                    page: 1,
                    per_page: 20,
                    total_pages: 1,
                })
            });
        photo_repository.expect_update_one().times(0);

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
            .expect_find_path()
            .withf(|p: &PaginationFilter, f: &PhotoFindPathFilters| {
                p.page == 1
                    && p.per_page == 20
                    && f.has_embedding == Some(false)
                    && f.has_face_detection_completed == None
            })
            .times(1)
            .returning(|_, __| Err(anyhow!("Repository error")));

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
            .expect_find_path()
            .times(1)
            .returning(|_, __| {
                Ok(PaginatedPhotoPaths {
                    items: vec![PhotoPath {
                        id: 1,
                        path: "test.jpg".to_string(),
                    }],
                    total: 1,
                    page: 1,
                    per_page: 20,
                    total_pages: 1,
                })
            });

        photo_repository
            .expect_update_one()
            .withf(|id: &i32, _| *id == 1)
            .times(1)
            .returning(|_, __| Err(anyhow!("Failed to update embeddings in database")));

        let mut image_embedder = MockImageEmbedder::new();

        image_embedder
            .expect_embed()
            .times(1)
            .returning(|_| Ok(vec![vec![0.]]));

        let mut photo_embedder_service =
            PhotoEmbedderService::new(photo_repository, image_embedder, NoOpProgressReporter);
        let result = photo_embedder_service.embed();

        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to update embeddings in database"
        );
    }
}
