use picasa_core::{
    models::NewPerson,
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
pub async fn load_face_image(face_id: i32, state: State<'_, AppState>) -> Result<Vec<u8>, ()> {
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let mut face_service = FaceService::new(face_repository);
    let face_with_photo = face_service.get_with_photo(face_id).unwrap().unwrap();

    let bbox = BoundingBox {
        x: face_with_photo.face.bbox_x,
        y: face_with_photo.face.bbox_y,
        width: face_with_photo.face.bbox_width,
        height: face_with_photo.face.bbox_height,
    };

    let img_service = ImageService::new(Path::new("../../cache_dir").to_path_buf());
    let data = img_service
        .crop_image(face_with_photo.photo_path, bbox)
        .unwrap();

    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub fn create_person_from_faces(
    person_name: &str,
    face_ids: Vec<i32>,
    state: State<'_, AppState>,
) -> Result<(), ()> {
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let person_repository = PgPersonRepository::new(state.db_pool.clone());

    let mut person_service = PersonService::new(person_repository, face_repository);

    person_service
        .create_from_faces(
            NewPerson {
                name: person_name.to_string(),
            },
            face_ids,
        )
        .unwrap();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_pending_manual_reviews(
    state: State<'_, AppState>,
) -> Result<Vec<PendingFaceReview>, ()> {
    let person_repository = PgPersonRepository::new(state.db_pool.clone());
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let mut face_recognition_service = FaceRecognitionService::new(
        face_repository,
        person_repository,
        NoOpProgressReporter,
        None,
    );

    let result = face_recognition_service.recognize_faces(true).unwrap();

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
