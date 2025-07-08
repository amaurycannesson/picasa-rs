use picasa_core::{
    repositories::{PgGeoRepository, PgPhotoRepository},
    services::{embedders::ClipTextEmbedder, PhotoSearchService},
};
#[cfg(debug_assertions)]
use std::path::Path;
use tauri::State;

use crate::{
    services::image::ImageService,
    types::{PaginatedPhotos, PhotoSearchParams},
    AppState,
};

#[tauri::command]
#[specta::specta]
pub async fn load_photo(path: &str) -> Result<Vec<u8>, ()> {
    let img_service = ImageService::new(Path::new("../../cache_dir").to_path_buf());
    let data = img_service.get_thumbnail(path).await.unwrap();
    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub fn search_photos(
    params: PhotoSearchParams,
    state: State<'_, AppState>,
) -> Result<PaginatedPhotos, ()> {
    let photo_repository = PgPhotoRepository::new(state.db_pool.clone());
    let geo_repository = PgGeoRepository::new(state.db_pool.clone());
    let text_embedder = ClipTextEmbedder::new().unwrap();
    let mut photo_search = PhotoSearchService::new(photo_repository, geo_repository, text_embedder);

    let result = photo_search.search(params.into()).unwrap();

    Ok(PaginatedPhotos::from(result))
}
