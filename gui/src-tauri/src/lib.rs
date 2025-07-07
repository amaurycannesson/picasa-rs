use picasa_core::{
    database,
    repositories::{PgGeoRepository, PgPhotoRepository},
    services::{embedders::ClipTextEmbedder, PhotoSearchService},
};
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use std::path::Path;
use tauri_specta::{collect_commands, Builder};

use crate::{
    services::image::ImageService,
    types::{PaginatedPhotos, PhotoSearchParams},
};

pub mod services;
pub mod types;

#[tauri::command]
#[specta::specta]
async fn load_photo(path: &str) -> Result<Vec<u8>, ()> {
    let img_service = ImageService::new(Path::new("../../cache_dir").to_path_buf());
    let data = img_service.get_thumbnail(path).await.unwrap();
    Ok(data)
}

#[tauri::command]
#[specta::specta]
fn search_photos(params: PhotoSearchParams) -> Result<PaginatedPhotos, ()> {
    let pool = database::create_pool();

    let photo_repository = PgPhotoRepository::new(pool.clone());
    let geo_repository = PgGeoRepository::new(pool.clone());
    let text_embedder = ClipTextEmbedder::new().unwrap();
    let mut photo_search = PhotoSearchService::new(photo_repository, geo_repository, text_embedder);

    let result = photo_search.search(params.into()).unwrap();

    Ok(PaginatedPhotos::from(result))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder =
        Builder::<tauri::Wry>::new().commands(collect_commands![search_photos, load_photo]);

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
