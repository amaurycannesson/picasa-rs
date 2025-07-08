use picasa_core::{
    repositories::{PgFaceRepository, PgPersonRepository},
    services::{
        face_recognition::RecognitionAction, FaceRecognitionService, FaceService, PersonService,
    },
    utils::progress_reporter::NoOpProgressReporter,
};
use std::path::Path;
use tauri::State;

use crate::{
    services::image::{BoundingBox, ImageService},
    types::PendingFaceReview,
    AppState,
};

#[tauri::command]
#[specta::specta]
pub async fn load_face_image(face_id: i32, state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let mut face_service = FaceService::new(face_repository);
    let face_with_photo = face_service
        .get_with_photo(face_id)
        .map_err(|e| format!("Failed to get face with photo: {}", e))?
        .ok_or_else(|| format!("Face with id {} not found", face_id))?;

    let bbox = BoundingBox {
        x: face_with_photo.face.bbox_x,
        y: face_with_photo.face.bbox_y,
        width: face_with_photo.face.bbox_width,
        height: face_with_photo.face.bbox_height,
    };

    let img_service = ImageService::new(Path::new("../../cache_dir").to_path_buf());
    let data = img_service
        .crop_image(face_with_photo.photo_path, bbox)
        .map_err(|e| format!("Failed to crop image: {}", e))?;

    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub async fn get_pending_manual_reviews(
    state: State<'_, AppState>,
) -> Result<Vec<PendingFaceReview>, String> {
    let person_repository = PgPersonRepository::new(state.db_pool.clone());
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let mut face_recognition_service = FaceRecognitionService::new(
        face_repository,
        person_repository,
        NoOpProgressReporter,
        None,
    );

    let result = face_recognition_service
        .recognize_faces(true)
        .map_err(|e| format!("Failed to recognize faces: {}", e))?;

    Ok(result
        .results
        .into_iter()
        .filter_map(|r| {
            if let RecognitionAction::ManualReview {
                face_count,
                confidence,
                ..
            } = r.action
            {
                Some(PendingFaceReview {
                    cluster_id: r.cluster_id,
                    face_ids: r.face_ids,
                    face_count,
                    confidence,
                })
            } else {
                None
            }
        })
        .collect())
}
