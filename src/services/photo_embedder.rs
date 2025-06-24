use pgvector::Vector;
use std::time::Instant;

use crate::{
    models::photo::PhotoEmbedding, photo_repository::PhotoRepository,
    services::embedders::image::ImageEmbedder, utils::progress_reporter::ProgressReporter,
};

/// Compute embeddings
pub fn embed(repo: &mut impl PhotoRepository, progress: &dyn ProgressReporter) {
    let image_embedder = ImageEmbedder::new().unwrap();

    let start = Instant::now();
    let per_page = 20i64;
    let mut page = 1i64;
    let mut total_processed = 0usize;

    loop {
        // Get the next batch of photos without embeddings
        let paginated_paths = match repo.list_paths_without_embedding(page, per_page) {
            Ok(paths) => paths,
            Err(e) => {
                progress.finish_with_message(format!("✗ Error fetching photos: {}", e));
                return;
            }
        };

        // If no more photos to process, break
        if paginated_paths.paths.is_empty() {
            break;
        }

        progress.set_message(format!(
            "Processing batch {} ({} photos remaining)",
            page,
            paginated_paths.total - (page - 1) * per_page
        ));

        // Compute embeddings for this batch
        let embeddings_data = match image_embedder.embed(&paginated_paths.paths) {
            Ok(tensor) => tensor,
            Err(e) => {
                progress.finish_with_message(format!("✗ Error computing embeddings: {}", e));
                return;
            }
        };

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
        match repo.update_embeddings(photo_embeddings) {
            Ok(updated_count) => {
                total_processed += updated_count;
                progress.set_message(format!(
                    "Processed {} photos (batch {}, {} total)",
                    updated_count, page, total_processed
                ));
            }
            Err(e) => {
                progress.finish_with_message(format!("✗ Error updating embeddings: {}", e));
                return;
            }
        }

        page += 1;
    }

    let duration = start.elapsed();
    progress.finish_with_message(format!(
        "✓ Processed embeddings for {} photos in {:.2?}",
        total_processed, duration
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::photo::PaginatedPaths, photo_repository::MockPhotoRepository,
        utils::progress_reporter::NoOpProgressReporter,
    };
    use mockall::predicate::*;

    #[test]
    fn test_process_embeddings_no_photos() {
        let mut mock_repo = MockPhotoRepository::new();

        mock_repo
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

        embed(&mut mock_repo, &NoOpProgressReporter);

        mock_repo.expect_update_embeddings().times(0);
    }
}
