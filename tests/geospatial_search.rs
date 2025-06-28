use std::fs;

use diesel::connection::SimpleConnection;
use picasa_rs::{
    photo_repository::PgPhotoRepository, services::geospatial_search::GeospatialSearchService,
};
use serial_test::serial;

mod db;
use db::get_pool;

#[test]
#[serial]
fn test_should_find_photo_in_vietnam() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let photo_repository = PgPhotoRepository::new(pool);
    let mut semantic_search_service = GeospatialSearchService::new(photo_repository);
    let results = semantic_search_service
        .search("vietnam")
        .expect("Search failed");

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].path, "tests/data/images/sub/desk_vietnam.heic");
    assert_eq!(
        results[1].path,
        "tests/data/images/sub/sub/building_vietnam.jpg"
    );
}
