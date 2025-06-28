use std::fs;

use diesel::connection::SimpleConnection;
use picasa_rs::{
    photo_repository::PgPhotoRepository,
    services::{embedders::text::ClipTextEmbedder, semantic_search::SemanticSearchService},
};
use serial_test::serial;

mod db;
use db::get_pool;

#[test]
#[serial]
fn test_should_find_photo_with_desk() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let photo_repository = PgPhotoRepository::new(pool);
    let text_embedder = ClipTextEmbedder::new().expect("Failed to create embedder");
    let mut semantic_search_service = SemanticSearchService::new(photo_repository, text_embedder);
    let results = semantic_search_service
        .search("desk", None, None)
        .expect("Search failed");

    assert_eq!(results[0].photo.path, "data/images/sub/desk_vietnam.heic");
}
