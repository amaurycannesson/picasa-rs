use std::sync::Once;

use diesel::{Connection, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use pgvector::Vector;
use picasa_rs::{
    database::schema,
    database::{self},
    models::photo::{Photo, PhotoEmbedding},
    photo_repository::{PgPhotoRepository, PhotoRepository},
};

static INIT: Once = Once::new();

fn establish_connection() -> PgConnection {
    dotenvy::from_filename(".env.test").ok();

    let mut conn = database::establish_connection();

    INIT.call_once(|| {
        database::run_migrations(&mut conn);
    });

    diesel::delete(schema::photos::table)
        .execute(&mut conn)
        .expect("Failed to clean test data");

    conn
}

#[test]
fn test_should_insert_batch() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, diesel::result::Error, _>(|conn| {
        let photos = vec![
            Photo {
                path: "path1".to_string(),
                ..Default::default()
            },
            Photo {
                path: "path2".to_string(),
                ..Default::default()
            },
        ];

        let mut repo = PgPhotoRepository::new(conn);
        let count = repo.insert_batch(photos)?;

        let photos: Vec<Photo> = schema::photos::table
            .select(Photo::as_select())
            .load(conn)?;
        assert_eq!(count, 2);
        assert_eq!(photos[0].path, "path1");
        assert_eq!(photos[1].path, "path2");
        Ok(())
    });
}

#[test]
fn test_should_handle_insert_conflicts_with_upsert() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, diesel::result::Error, _>(|conn| {
        let original_photo = Photo {
            path: "test/photo.jpg".to_string(),
            file_name: "photo.jpg".to_string(),
            file_size: 1000,
            hash: Some("original_hash".to_string()),
            camera_make: Some("Canon".to_string()),
            ..Default::default()
        };

        let mut repo = PgPhotoRepository::new(conn);
        let count = repo.insert_batch(vec![original_photo.clone()])?;
        assert_eq!(count, 1);

        let updated_photo = Photo {
            path: "test/photo.jpg".to_string(),
            file_name: "photo.jpg".to_string(),
            file_size: 2000,
            hash: Some("updated_hash".to_string()),
            camera_make: Some("Nikon".to_string()),
            camera_model: Some("D850".to_string()),
            ..Default::default()
        };

        let count = repo.insert_batch(vec![updated_photo])?;
        assert_eq!(count, 1);

        let photos: Vec<Photo> = schema::photos::table
            .select(Photo::as_select())
            .load(conn)?;

        assert_eq!(photos.len(), 1);
        let photo = &photos[0];
        assert_eq!(photo.path, "test/photo.jpg");
        assert_eq!(photo.file_name, "photo.jpg");
        assert_eq!(photo.file_size, 2000);
        assert_eq!(photo.hash, Some("updated_hash".to_string()));
        assert_eq!(photo.camera_make, Some("Nikon".to_string()));
        assert_eq!(photo.camera_model, Some("D850".to_string()));

        Ok(())
    });
}

#[test]
fn test_should_preserve_embeddings_on_conflict() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, diesel::result::Error, _>(|conn| {
        let embedding_vector = Vector::from(vec![0.1_f32; 512]);
        let original_photo = Photo {
            path: "test/photo_with_embedding.jpg".to_string(),
            embedding: Some(embedding_vector.clone()),
            ..Default::default()
        };

        let mut repo = PgPhotoRepository::new(conn);
        repo.insert_batch(vec![original_photo])?;

        let updated_photo = Photo {
            path: "test/photo_with_embedding.jpg".to_string(),
            embedding: None,
            ..Default::default()
        };

        repo.insert_batch(vec![updated_photo])?;

        let photos: Vec<Photo> = schema::photos::table
            .select(Photo::as_select())
            .load(conn)?;

        assert_eq!(photos.len(), 1);
        let photo = &photos[0];
        assert!(photo.embedding.is_some());
        assert_eq!(
            photo.embedding.as_ref().unwrap().as_slice(),
            embedding_vector.as_slice()
        );

        Ok(())
    });
}

#[test]
fn test_should_clear_embedding_on_hash_change() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, diesel::result::Error, _>(|conn| {
        let embedding_vector = Vector::from(vec![0.1_f32; 512]);
        let original_photo = Photo {
            path: "test/photo_with_embedding.jpg".to_string(),
            hash: Some("original_hash".to_string()),
            embedding: Some(embedding_vector.clone()),
            ..Default::default()
        };

        let mut repo = PgPhotoRepository::new(conn);
        repo.insert_batch(vec![original_photo])?;

        let updated_photo = Photo {
            path: "test/photo_with_embedding.jpg".to_string(),
            hash: Some("new_hash".to_string()),
            embedding: Some(embedding_vector.clone()),
            ..Default::default()
        };

        repo.insert_batch(vec![updated_photo])?;

        let photos: Vec<Photo> = schema::photos::table
            .select(Photo::as_select())
            .load(conn)?;

        assert!(photos[0].embedding.is_none());

        Ok(())
    });
}

#[test]
fn test_should_list_paths_without_embedding() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, diesel::result::Error, _>(|conn| {
        let photos = vec![
            Photo {
                path: "path1".to_string(),
                ..Default::default()
            },
            Photo {
                path: "path2".to_string(),
                embedding: Some(Vector::from(vec![0.1_f32; 512])),
                ..Default::default()
            },
        ];
        diesel::insert_into(schema::photos::table)
            .values(photos)
            .execute(conn)?;

        let mut repo = PgPhotoRepository::new(conn);
        let paginated_paths = repo.list_paths_without_embedding(1, 10)?;

        assert_eq!(paginated_paths.paths, vec!["path1"]);
        Ok(())
    });
}

#[test]
fn test_should_update_embeddings() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, diesel::result::Error, _>(|conn| {
        let photo = Photo {
            path: "test/photo.jpg".to_string(),
            file_name: "photo.jpg".to_string(),
            ..Default::default()
        };
        diesel::insert_into(schema::photos::table)
            .values(photo.clone())
            .execute(conn)?;

        let embedding_vector = Vector::from(vec![0.1_f32; 512]);
        let photo_embedding = PhotoEmbedding {
            path: photo.path,
            embedding: Some(embedding_vector.clone()),
        };

        let mut repo = PgPhotoRepository::new(conn);
        let update_count = repo.update_embeddings(vec![photo_embedding])?;

        let photos: Vec<Photo> = schema::photos::table
            .select(Photo::as_select())
            .load(conn)?;

        assert_eq!(update_count, 1);
        assert_eq!(
            photos[0].embedding.as_ref().unwrap().as_slice(),
            embedding_vector.as_slice()
        );
        Ok(())
    });
}
