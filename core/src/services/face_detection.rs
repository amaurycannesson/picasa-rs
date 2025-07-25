use std::time::Instant;

use anyhow::{Context, Result};
use pgvector::Vector;
use serde::{Deserialize, Serialize};

use crate::{
    config::FaceDetectionServerConfig,
    models::{NewFace, PaginationFilter, UpdatedPhoto},
    repositories::{
        PhotoFindPathFilters, face::repository::FaceRepository, photo::repository::PhotoRepository,
    },
    utils::progress_reporter::ProgressReporter,
};

#[derive(Serialize)]
struct DetectFacesRequest {
    image_path: String,
}

#[derive(Deserialize)]
struct DetectFacesResponse {
    faces: Vec<DetectedFace>,
}

#[derive(Deserialize)]
struct DetectedFace {
    confidence: f32,
    embedding: Vec<f32>,
    bbox: BoundingBox,
    gender: String,
}

#[derive(Deserialize)]
struct BoundingBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub struct FaceDetectionService<PR: PhotoRepository, FR: FaceRepository, P: ProgressReporter> {
    photo_repository: PR,
    face_repository: FR,
    progress_reporter: P,
    http_client: reqwest::blocking::Client,
    face_detection_url: String,
}

impl<PR: PhotoRepository, FR: FaceRepository, P: ProgressReporter> FaceDetectionService<PR, FR, P> {
    pub fn new(
        photo_repository: PR,
        face_repository: FR,
        progress_reporter: P,
        face_detection_config: &FaceDetectionServerConfig,
    ) -> Self {
        let face_detection_url = format!(
            "http://{}:{}/detect-faces",
            face_detection_config.host, face_detection_config.port
        );
        Self {
            photo_repository,
            face_repository,
            progress_reporter,
            http_client: reqwest::blocking::Client::new(),
            face_detection_url,
        }
    }

    /// Detects faces in photos that haven't been processed yet.
    pub fn detect_faces(&mut self) -> Result<usize> {
        let start = Instant::now();
        let mut total_processed = 0usize;

        loop {
            let paginated_paths = self
                .photo_repository
                .find_path(
                    PaginationFilter {
                        page: 1,
                        per_page: 20,
                    },
                    PhotoFindPathFilters {
                        has_face_detection_completed: Some(false),
                        ..Default::default()
                    },
                )
                .context("Failed to fetch photos without face detection")?;

            if paginated_paths.items.is_empty() {
                break;
            }

            self.progress_reporter.set_message(format!(
                "Processing photos: {} total, {} remaining",
                total_processed, paginated_paths.total
            ));

            for photo_path in &paginated_paths.items {
                let photo_id = photo_path.id;
                let path = photo_path.path.clone();
                match self.detect_faces_for_photo(&path) {
                    Ok(detected_faces) => {
                        for detected_face in detected_faces {
                            let new_face =
                                convert_detected_face_to_new_face(detected_face, photo_id);
                            self.face_repository
                                .insert_one(new_face)
                                .context(format!("Failed to insert face for photo: {}", path))?;
                        }

                        self.photo_repository
                            .update_one(
                                photo_id,
                                UpdatedPhoto {
                                    face_detection_completed: Some(true),
                                    ..Default::default()
                                },
                            )
                            .context("Failed to update face detection completed status")?;
                    }
                    Err(_) => {
                        // TODO: Fix infinite loop if only errors
                    }
                }
            }

            total_processed += paginated_paths.items.len();
        }

        let duration = start.elapsed();
        self.progress_reporter.finish_with_message(format!(
            "✓ Processed face detection for {} photos in {:.2?}",
            total_processed, duration
        ));

        Ok(total_processed)
    }

    fn detect_faces_for_photo(&self, image_path: &str) -> Result<Vec<DetectedFace>> {
        let request = DetectFacesRequest {
            image_path: image_path.to_string(),
        };

        let response = self
            .http_client
            .post(&self.face_detection_url)
            .json(&request)
            .send()
            .context("Failed to send face detection request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "Face detection API returned error {}: {}",
                status,
                error_text
            ));
        }

        let face_response: DetectFacesResponse = response
            .json()
            .context("Failed to parse face detection response")?;

        Ok(face_response.faces)
    }
}

fn convert_detected_face_to_new_face(detected_face: DetectedFace, photo_id: i32) -> NewFace {
    NewFace {
        photo_id,
        person_id: None,
        bbox_x: detected_face.bbox.x,
        bbox_y: detected_face.bbox.y,
        bbox_width: detected_face.bbox.width,
        bbox_height: detected_face.bbox.height,
        confidence: detected_face.confidence,
        gender: Some(detected_face.gender),
        embedding: Some(Vector::from(detected_face.embedding)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::PaginatedPhotoPaths,
        repositories::{
            face::repository::MockFaceRepository, photo::repository::MockPhotoRepository,
        },
        utils::progress_reporter::NoOpProgressReporter,
    };
    use anyhow::anyhow;

    #[test]
    fn test_detect_faces_no_photos() {
        let mut photo_repository = MockPhotoRepository::new();
        photo_repository
            .expect_find_path()
            .withf(|p: &PaginationFilter, f: &PhotoFindPathFilters| {
                p.page == 1
                    && p.per_page == 20
                    && f.has_embedding == None
                    && f.has_face_detection_completed == Some(false)
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

        let mut face_repository = MockFaceRepository::new();
        face_repository.expect_insert_one().times(0);

        let face_detection_config = FaceDetectionServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        let mut face_detection_service = FaceDetectionService::new(
            photo_repository,
            face_repository,
            NoOpProgressReporter,
            &face_detection_config,
        );
        let result = face_detection_service.detect_faces();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_should_return_error_when_photo_repository_fails() {
        let mut photo_repository = MockPhotoRepository::new();

        photo_repository
            .expect_find_path()
            .withf(|p: &PaginationFilter, f: &PhotoFindPathFilters| {
                p.page == 1
                    && p.per_page == 20
                    && f.has_embedding == None
                    && f.has_face_detection_completed == Some(false)
            })
            .times(1)
            .returning(|_, __| Err(anyhow!("Repository error")));

        let mut face_repository = MockFaceRepository::new();
        face_repository.expect_insert_one().times(0);

        let face_detection_config = FaceDetectionServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        let mut face_detection_service = FaceDetectionService::new(
            photo_repository,
            face_repository,
            NoOpProgressReporter,
            &face_detection_config,
        );
        let result = face_detection_service.detect_faces();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to fetch photos without face detection"
        );
    }
}
