use std::time::Instant;

use anyhow::{Context, Result};
use ignore;

use crate::models::NewPhoto;
use crate::repositories::PhotoRepository;
use crate::utils::{self, progress_reporter::ProgressReporter};

/// Scans photos in the given path and inserts them into the repository.
pub fn scan(
    path: &str,
    photo_repository: &mut dyn PhotoRepository,
    with_exif: bool,
    with_hash: bool,
    progress: &dyn ProgressReporter,
) -> Result<usize> {
    let start = Instant::now();
    let photos: Vec<NewPhoto> = ignore::WalkBuilder::new(path)
        .git_ignore(false)
        .build()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "heic"))
                .unwrap_or(false)
        })
        .filter_map(|entry| match NewPhoto::new(entry.path()) {
            Ok(mut new_photo) => {
                if with_hash {
                    if let Ok(hash) = utils::compute_file_hash(&new_photo.path) {
                        new_photo = new_photo.with_hash(hash);
                    }
                }

                if with_exif {
                    if let Some(exif) = utils::extract_exif(&new_photo.path) {
                        new_photo = new_photo.with_exif(exif);
                    }
                }

                progress.set_message(format!("{}", new_photo.path));

                Some(new_photo)
            }
            Err(err) => {
                progress.set_message(format!(
                    "Error processing {}: {}",
                    entry.path().display(),
                    err
                ));
                None
            }
        })
        .collect();

    let count = photo_repository
        .insert_batch(photos)
        .context("Failed to insert photos into repository")?;

    let duration = start.elapsed();
    progress.finish_with_message(format!("✓ Scanned {} photos in {:.2?}", count, duration));

    Ok(count)
}

#[cfg(test)]
mod tests {
    use crate::{
        repositories::MockPhotoRepository, utils::progress_reporter::NoOpProgressReporter,
    };

    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_should_scan_photos() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        File::create(temp_path.join("photo1.jpg")).unwrap();
        File::create(temp_path.join("photo2.JPG")).unwrap();
        File::create(temp_path.join("photo3.jpeg")).unwrap();
        File::create(temp_path.join("photo4.png")).unwrap();
        File::create(temp_path.join("not_a_photo.txt")).unwrap();

        std::fs::create_dir(temp_path.join("subdir")).unwrap();
        File::create(temp_path.join("subdir/photo5.jpg")).unwrap();
        File::create(temp_path.join("subdir/not_a_photo2.txt")).unwrap();

        let mut mock = MockPhotoRepository::new();
        mock.expect_insert_batch()
            .returning(|new_photos| Ok(new_photos.len()));

        let photo_count = scan(
            temp_path.to_str().unwrap(),
            &mut mock,
            false,
            false,
            &NoOpProgressReporter,
        )
        .unwrap();
        assert_eq!(photo_count, 5, "Expected 5 photos to be scanned");
    }

    #[test]
    fn test_should_compute_file_hash() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        let test_file = temp_path.join("test.txt");

        std::fs::write(&test_file, b"Hello, World!").unwrap();

        let hash = utils::compute_file_hash(test_file.to_str().unwrap()).unwrap();

        assert_eq!(
            hash,
            "288a86a79f20a3d6dccdca7713beaed178798296bdfa7913fa2a62d9727bf8f8"
        );
    }
}
