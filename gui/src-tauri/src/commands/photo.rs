use picasa_core::{
    repositories::{PgGeoRepository, PgPersonRepository, PgPhotoRepository},
    services::{embedders::ClipTextEmbedder, PhotoSearchService},
};
#[cfg(debug_assertions)]
use std::path::Path;
use tauri::State;

use crate::{
    services::image::ImageService,
    types::{PaginatedPhotos, PhotoSearchOptions, PhotoSearchParams},
    AppState,
};

#[tauri::command]
#[specta::specta]
pub async fn load_photo(path: &str) -> Result<Vec<u8>, String> {
    let img_service = ImageService::new(Path::new("../../cache_dir").to_path_buf());

    img_service
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
    let photo_repository = PgPhotoRepository::new(state.db_pool.clone());
    let geo_repository = PgGeoRepository::new(state.db_pool.clone());
    let text_embedder = ClipTextEmbedder::new().unwrap();
    let mut photo_search = PhotoSearchService::new(
        photo_repository,
        geo_repository,
        person_repository,
        text_embedder,
    );

    photo_search
        .search(params.into())
        .map(PaginatedPhotos::from)
        .map_err(|e| format!("Failed to search photos: {}", e))
}

#[tauri::command]
#[specta::specta]
pub async fn get_search_options(
    state: State<'_, AppState>,
) -> Result<PhotoSearchOptions, String> {
    let person_repository = PgPersonRepository::new(state.db_pool.clone());
    let photo_repository = PgPhotoRepository::new(state.db_pool.clone());
    let geo_repository = PgGeoRepository::new(state.db_pool.clone());
    let text_embedder = ClipTextEmbedder::new().unwrap();
    let mut photo_search = PhotoSearchService::new(
        photo_repository,
        geo_repository,
        person_repository,
        text_embedder,
    );

    photo_search
        .get_search_options()
        .map(PhotoSearchOptions::from)
        .map_err(|e| format!("Failed to get search options: {}", e))
}
