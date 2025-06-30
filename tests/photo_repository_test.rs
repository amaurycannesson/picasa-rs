use chrono::{DateTime, NaiveDate, Utc};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, connection::SimpleConnection};
use pgvector::Vector;
use picasa_rs::{
    database::schema,
    models::{
        NewPhoto, PaginationFilter, {Photo, PhotoEmbedding},
    },
    repositories::{PgPhotoRepository, PhotoFindFilters, PhotoRepository},
    services::embedders::text::{ClipTextEmbedder, TextEmbedder},
};
use serial_test::serial;

mod db;
use db::get_pool;

fn load_photos(conn: &mut PgConnection) -> Vec<Photo> {
    schema::photos::table
        .select(Photo::as_select())
        .load(conn)
        .expect("Failed to load photos")
}

#[test]
#[serial]
fn test_should_insert_batch() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let photos = vec![
        NewPhoto {
            path: "path1".to_string(),
            ..Default::default()
        },
        NewPhoto {
            path: "path2".to_string(),
            ..Default::default()
        },
    ];

    let mut repo = PgPhotoRepository::new(pool);
    let count = repo.insert_batch(photos).expect("Failed to insert photos");

    let photos = load_photos(&mut conn);
    let paths = vec![photos[0].path.to_string(), photos[1].path.to_string()];

    assert_eq!(count, 2);
    assert!(paths.contains(&"path1".to_string()));
    assert!(paths.contains(&"path2".to_string()));
}

#[test]
#[serial]
fn test_should_handle_insert_conflicts_with_upsert() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let original_photo = NewPhoto {
        path: "test/photo.jpg".to_string(),
        file_name: "photo.jpg".to_string(),
        file_size: 1000,
        hash: Some("original_hash".to_string()),
        camera_make: Some("Canon".to_string()),
        ..Default::default()
    };

    let mut repo = PgPhotoRepository::new(pool);
    let count = repo
        .insert_batch(vec![original_photo.clone()])
        .expect("Failed to insert original photo");

    assert_eq!(count, 1);

    let updated_photo = NewPhoto {
        path: "test/photo.jpg".to_string(),
        file_name: "photo.jpg".to_string(),
        file_size: 2000,
        hash: Some("updated_hash".to_string()),
        camera_make: Some("Nikon".to_string()),
        camera_model: Some("D850".to_string()),
        ..Default::default()
    };

    let count = repo
        .insert_batch(vec![updated_photo])
        .expect("Failed to insert updated photo");

    assert_eq!(count, 1);

    let photos = load_photos(&mut conn);

    assert_eq!(photos.len(), 1);

    let photo = &photos[0];

    assert_eq!(photo.path, "test/photo.jpg");
    assert_eq!(photo.file_name, "photo.jpg");
    assert_eq!(photo.file_size, 2000);
    assert_eq!(photo.hash, Some("updated_hash".to_string()));
    assert_eq!(photo.camera_make, Some("Nikon".to_string()));
    assert_eq!(photo.camera_model, Some("D850".to_string()));
}

#[test]
#[serial]
fn test_should_preserve_embeddings_on_conflict() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let embedding_vector = Vector::from(vec![0.1_f32; 512]);
    let original_photo = NewPhoto {
        path: "test/photo_with_embedding.jpg".to_string(),
        embedding: Some(embedding_vector.clone()),
        ..Default::default()
    };

    let mut repo = PgPhotoRepository::new(pool);
    repo.insert_batch(vec![original_photo])
        .expect("Failed to insert original photo");

    let updated_photo = NewPhoto {
        path: "test/photo_with_embedding.jpg".to_string(),
        embedding: None,
        ..Default::default()
    };

    repo.insert_batch(vec![updated_photo])
        .expect("Failed to insert updated photo");

    let photos = load_photos(&mut conn);

    assert_eq!(photos.len(), 1);
    let photo = &photos[0];
    assert!(photo.embedding.is_some());
    assert_eq!(
        photo.embedding.as_ref().unwrap().as_slice(),
        embedding_vector.as_slice()
    );
}

#[test]
#[serial]
fn test_should_clear_embedding_on_hash_change() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let embedding_vector = Vector::from(vec![0.1_f32; 512]);
    let original_photo = NewPhoto {
        path: "test/photo_with_embedding.jpg".to_string(),
        hash: Some("original_hash".to_string()),
        embedding: Some(embedding_vector.clone()),
        ..Default::default()
    };

    let mut repo = PgPhotoRepository::new(pool);
    repo.insert_batch(vec![original_photo])
        .expect("Failed to insert original photo");

    let updated_photo = NewPhoto {
        path: "test/photo_with_embedding.jpg".to_string(),
        hash: Some("new_hash".to_string()),
        embedding: Some(embedding_vector.clone()),
        ..Default::default()
    };

    repo.insert_batch(vec![updated_photo])
        .expect("Failed to insert updated photo");

    let photos = load_photos(&mut conn);

    assert!(photos[0].embedding.is_none());
}

#[test]
#[serial]
fn test_should_list_paths_without_embedding() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let photos = vec![
        NewPhoto {
            path: "path1".to_string(),
            ..Default::default()
        },
        NewPhoto {
            path: "path2".to_string(),
            embedding: Some(Vector::from(vec![0.1_f32; 512])),
            ..Default::default()
        },
    ];
    diesel::insert_into(schema::photos::table)
        .values(photos)
        .execute(&mut conn)
        .expect("Failed to insert photos");

    let mut repo = PgPhotoRepository::new(pool);
    let paginated_paths = repo
        .list_paths_without_embedding(1, 10)
        .expect("Failed to list paths");

    assert_eq!(paginated_paths.paths, vec!["path1"]);
}

#[test]
#[serial]
fn test_should_update_embeddings() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let photo = NewPhoto {
        path: "test/photo.jpg".to_string(),
        file_name: "photo.jpg".to_string(),
        ..Default::default()
    };
    diesel::insert_into(schema::photos::table)
        .values(photo.clone())
        .execute(&mut conn)
        .expect("Failed to insert photo");

    let embedding_vector = Vector::from(vec![0.1_f32; 512]);
    let photo_embedding = PhotoEmbedding {
        path: photo.path,
        embedding: Some(embedding_vector.clone()),
    };

    let mut repo = PgPhotoRepository::new(pool);
    let update_count = repo
        .update_embeddings(vec![photo_embedding])
        .expect("Failed to update embeddings");

    let photos = load_photos(&mut conn);

    assert_eq!(update_count, 1);
    assert_eq!(
        photos[0].embedding.as_ref().unwrap().as_slice(),
        embedding_vector.as_slice()
    );
}

#[test]
#[serial]
fn test_should_find_photos_with_pagination() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    // Load test data
    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);

    // Create search filters with just pagination options
    let filters = PaginationFilter {
        page: 1,
        per_page: 2,
    };

    // Search photos with pagination
    let result = repo
        .find(filters, PhotoFindFilters::default())
        .expect("Failed to search photos");

    // Verify pagination works correctly
    assert_eq!(result.photos.len(), 2); // Should return 2 photos per page
    assert_eq!(result.page, 1); // Should be on page 1
    assert_eq!(result.per_page, 2); // Should have 2 items per page
    assert!(result.total > 0); // Should have some total count
    assert!(result.total_pages > 0); // Should have some total pages
}

#[test]
#[serial]
fn test_should_find_photos_with_pagination_second_page() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);

    let page2_filters = PaginationFilter {
        page: 2,
        per_page: 1,
    };

    let page2_result = repo
        .find(page2_filters, PhotoFindFilters::default())
        .expect("Failed to search photos");

    assert_eq!(page2_result.photos.len(), 1);
    assert_eq!(page2_result.page, 2);
    assert_eq!(page2_result.per_page, 1);
    assert!(page2_result.total > 1);
    assert!(page2_result.total_pages > 1);

    let page1_filters = PaginationFilter {
        page: 1,
        per_page: 1,
    };

    let page1_result = repo
        .find(page1_filters, PhotoFindFilters::default())
        .expect("Failed to search photos for page 1");

    assert_ne!(
        page1_result.photos[0].path, page2_result.photos[0].path,
        "Photos on different pages should be different"
    );
}

#[test]
#[serial]
fn test_should_find_photos_by_date_range() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);

    let date_from = NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let date_to = NaiveDate::from_ymd_opt(2025, 12, 31)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();

    let filters = PhotoFindFilters {
        date_from: Some(DateTime::<Utc>::from_naive_utc_and_offset(date_from, Utc)),
        date_to: Some(DateTime::<Utc>::from_naive_utc_and_offset(date_to, Utc)),
        ..Default::default()
    };

    let result = repo
        .find(
            PaginationFilter {
                page: 1,
                per_page: 10,
            },
            filters,
        )
        .expect("Failed to search photos by date range");

    assert!(!result.photos.is_empty(), "Should find photos from 2025");
    for photo in &result.photos {
        if let Some(date_taken) = photo.date_taken_utc {
            assert!(
                date_taken >= DateTime::<Utc>::from_naive_utc_and_offset(date_from, Utc)
                    && date_taken <= DateTime::<Utc>::from_naive_utc_and_offset(date_to, Utc),
                "Photo date {} should be within the specified range",
                date_taken
            );
        }
    }
}

#[test]
#[serial]
fn test_should_find_photos_by_semantic_query() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);
    let text_embedder = ClipTextEmbedder::new().expect("Failed to create embedder");
    let text_embedding = text_embedder
        .embed("white building")
        .expect("Failed to create embedding");
    let filters = PhotoFindFilters {
        text_embedding: Some(text_embedding),
        threshold: Some(0.0),
        ..Default::default()
    };

    let result = repo
        .find(
            PaginationFilter {
                page: 1,
                per_page: 10,
            },
            filters,
        )
        .expect("Failed to search photos by semantic query");

    assert!(result.photos[0].path.contains("building_vietnam"));
}

#[test]
#[serial]
fn test_should_find_photos_by_country() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);

    let filters = PhotoFindFilters {
        country_id: Some(56),
        ..Default::default()
    };

    let result = repo
        .find(
            PaginationFilter {
                page: 1,
                per_page: 10,
            },
            filters,
        )
        .expect("Failed to search photos by country");

    assert_eq!(result.photos.len(), 2, "Should find 2 photos from Vietnam");
    assert_eq!(
        result.photos[0].path,
        "tests/data/images/sub/desk_vietnam.heic"
    );
    assert_eq!(
        result.photos[1].path,
        "tests/data/images/sub/sub/building_vietnam.jpg"
    );
}

#[test]
#[serial]
fn test_should_find_photos_with_combined_filters() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);

    let date_from = NaiveDate::from_ymd_opt(2025, 3, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let text_embedder = ClipTextEmbedder::new().expect("Failed to create embedder");
    let text_embedding = text_embedder
        .embed("white building")
        .expect("Failed to create embedding");
    let filters = PhotoFindFilters {
        text_embedding: Some(text_embedding),
        threshold: Some(0.23),
        country_id: Some(56),
        date_from: Some(DateTime::<Utc>::from_naive_utc_and_offset(date_from, Utc)),
        ..Default::default()
    };

    let result = repo
        .find(
            PaginationFilter {
                page: 1,
                per_page: 10,
            },
            filters,
        )
        .expect("Failed to search photos with combined filters");

    assert_eq!(
        result.photos.len(),
        1,
        "Should find 1 photo matching all criteria"
    );
    assert_eq!(
        result.photos[0].path, "tests/data/images/sub/sub/building_vietnam.jpg",
        "Should find the building photo from Vietnam"
    );
}

#[test]
#[serial]
fn test_should_find_photos_by_city() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);

    let filters = PhotoFindFilters {
        city_id: Some(1566083),
        ..Default::default()
    };

    let result = repo
        .find(
            PaginationFilter {
                page: 1,
                per_page: 10,
            },
            filters,
        )
        .expect("Failed to search photos by country");

    assert_eq!(
        result.photos.len(),
        2,
        "Should find 2 photos from Ho Chi Minh"
    );
    assert_eq!(
        result.photos[0].path,
        "tests/data/images/sub/desk_vietnam.heic"
    );
    assert_eq!(
        result.photos[1].path,
        "tests/data/images/sub/sub/building_vietnam.jpg"
    );
}
