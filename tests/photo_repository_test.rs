use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, connection::SimpleConnection};
use pgvector::Vector;
use picasa_rs::{
    database::schema,
    models::photo::{NewPhoto, Photo, PhotoEmbedding},
    photo_repository::{PgPhotoRepository, PhotoRepository},
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

    assert_eq!(count, 2);
    assert_eq!(photos[0].path, "path1");
    assert_eq!(photos[1].path, "path2");
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
fn test_should_find_photo_by_city() {
    use std::fs;

    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");

    let mut repo = PgPhotoRepository::new(pool);

    let photos = repo
        .find_by_city("Ho chi minh", Some(10000))
        .expect("Failed to find photos by city");

    assert_eq!(photos.len(), 2);
    assert_eq!(photos[0].path, "tests/data/images/sub/desk_vietnam.heic");
    assert_eq!(
        photos[1].path,
        "tests/data/images/sub/sub/building_vietnam.jpg"
    );
}
