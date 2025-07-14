use picasa_core::{
    repositories::{PgFaceRepository, PgGeoRepository, PgPersonRepository, PgPhotoRepository},
    services::{embedders::ClipTextEmbedder, PhotoSearchService},
};
#[cfg(debug_assertions)]
use tauri::State;

use crate::{
    types::{PaginatedPhotos, PhotoSearchOptions, PhotoSearchParams, PhotoWithFacesAndPeople},
    AppState,
};

#[tauri::command]
#[specta::specta]
pub async fn get_photo_with_faces_and_people(
    id: i32,
    state: State<'_, AppState>,
) -> Result<PhotoWithFacesAndPeople, String> {
    let person_repository = PgPersonRepository::new(state.db_pool.clone());
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let photo_repository = PgPhotoRepository::new(state.db_pool.clone());
    let geo_repository = PgGeoRepository::new(state.db_pool.clone());
    let text_embedder = ClipTextEmbedder::new().unwrap();
    let mut photo_search = PhotoSearchService::new(
        photo_repository,
        geo_repository,
        person_repository,
        face_repository,
        text_embedder,
    );

    let photo_with_faces_and_people = photo_search
        .get_photo_with_faces_and_people(id)
        .map_err(|e| format!("Failed to get photo with faces and people: {}", e))?
        .ok_or_else(|| format!("Photo with id {} not found", id))?;

    Ok(PhotoWithFacesAndPeople::from(photo_with_faces_and_people))
}

#[tauri::command]
#[specta::specta]
pub async fn load_photo(path: &str, state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    state
        .image_service
        .get_image(path)
        .await
        .map_err(|e| format!("Failed to load photo: {}", e))
}

#[tauri::command]
#[specta::specta]
pub async fn load_photo_thumbnail(
    path: &str,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    state
        .image_service
        .get_thumbnail(path)
        .await
        .map_err(|e| format!("Failed to load photo: {}", e))
}

#[tauri::command]
#[specta::specta]
pub async fn search_photos(
    params: PhotoSearchParams,
    state: State<'_, AppState>,
) -> Result<PaginatedPhotos, String> {
    let person_repository = PgPersonRepository::new(state.db_pool.clone());
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let photo_repository = PgPhotoRepository::new(state.db_pool.clone());
    let geo_repository = PgGeoRepository::new(state.db_pool.clone());
    let text_embedder = ClipTextEmbedder::new().unwrap();
    let mut photo_search = PhotoSearchService::new(
        photo_repository,
        geo_repository,
        person_repository,
        face_repository,
        text_embedder,
    );

    photo_search
        .search(params.into())
        .map(PaginatedPhotos::from)
        .map_err(|e| format!("Failed to search photos: {}", e))
}

#[tauri::command]
#[specta::specta]
pub async fn get_search_options(state: State<'_, AppState>) -> Result<PhotoSearchOptions, String> {
    let person_repository = PgPersonRepository::new(state.db_pool.clone());
    let face_repository = PgFaceRepository::new(state.db_pool.clone());
    let photo_repository = PgPhotoRepository::new(state.db_pool.clone());
    let geo_repository = PgGeoRepository::new(state.db_pool.clone());
    let text_embedder = ClipTextEmbedder::new().unwrap();
    let mut photo_search = PhotoSearchService::new(
        photo_repository,
        geo_repository,
        person_repository,
        face_repository,
        text_embedder,
    );

    photo_search
        .get_search_options()
        .map(PhotoSearchOptions::from)
        .map_err(|e| format!("Failed to get search options: {}", e))
}
