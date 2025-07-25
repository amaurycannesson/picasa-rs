use picasa_core::{
    repositories::{PgFaceRepository, PgGeoRepository, PgPersonRepository, PgPhotoRepository},
    services::{embedders::ClipTextEmbedder, photo_search::PhotoSearchService},
};
use serial_test::serial;

mod utils;
use utils::{get_pool, insert_photo_fixtures};

use crate::utils::load_config;

#[test]
#[serial]
fn test_should_return_search_options() {
    let config = load_config();
    let pool = get_pool();

    insert_photo_fixtures(pool.clone());

    let photo_repository = PgPhotoRepository::new(pool.clone());
    let geo_repository = PgGeoRepository::new(pool.clone());
    let person_repository = PgPersonRepository::new(pool.clone());
    let face_repository = PgFaceRepository::new(pool.clone());
    let text_embedder = ClipTextEmbedder::new(&config.clip_model).unwrap();

    let mut service = PhotoSearchService::new(
        photo_repository,
        geo_repository,
        person_repository,
        face_repository,
        text_embedder,
    );

    let options = service.get_search_options().unwrap();

    assert!(options.cities[0].id == 1566083); // Ho Chi Minh City
    assert!(options.cities[1].id == 1655087); // Vang Vieng

    assert!(options.countries[0].id == 68); // Laos
    assert!(options.countries[1].id == 56); // Vietnam
}
