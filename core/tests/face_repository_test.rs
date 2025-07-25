use diesel::RunQueryDsl;
use pgvector::Vector;
use picasa_core::{
    database::schema,
    models::{NewFace, PaginationFilter},
    repositories::{FaceFindFilters, FaceRepository, PgFaceRepository},
};
use serial_test::serial;

mod utils;
use utils::get_pool;
use utils::insert_photo_fixtures;

use crate::utils::load_photos;

#[test]
#[serial]
fn test_should_insert_one() {
    let pool = get_pool();

    insert_photo_fixtures(pool.clone());
    let photos = load_photos(pool.clone());

    let new_face = NewFace {
        photo_id: photos[0].id,
        person_id: None,
        bbox_x: 100,
        bbox_y: 200,
        bbox_width: 50,
        bbox_height: 60,
        confidence: 0.95,
        gender: Some("male".to_string()),
        embedding: Some(Vector::from(vec![0.1_f32; 512])),
    };

    let mut repo = PgFaceRepository::new(pool);
    let face = repo.insert_one(new_face).expect("Failed to insert face");

    assert_eq!(face.photo_id, photos[0].id);
    assert_eq!(face.bbox_x, 100);
    assert_eq!(face.bbox_y, 200);
    assert_eq!(face.bbox_width, 50);
    assert_eq!(face.bbox_height, 60);
    assert_eq!(face.confidence, 0.95);
    assert_eq!(face.gender, Some("male".to_string()));
    assert!(face.embedding.is_some());
    assert_eq!(face.embedding.as_ref().unwrap().as_slice(), &[0.1_f32; 512]);
}

#[test]
#[serial]
fn test_should_find_faces_by_photo_id() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    insert_photo_fixtures(pool.clone());
    let photos = load_photos(pool.clone());

    let faces = vec![
        NewFace {
            photo_id: photos[0].id,
            bbox_x: 100,
            bbox_y: 100,
            bbox_width: 50,
            bbox_height: 50,
            confidence: 0.9,
            ..Default::default()
        },
        NewFace {
            photo_id: photos[1].id,
            bbox_x: 200,
            bbox_y: 200,
            bbox_width: 60,
            bbox_height: 60,
            confidence: 0.8,
            ..Default::default()
        },
    ];

    diesel::insert_into(schema::faces::table)
        .values(faces)
        .execute(&mut conn)
        .expect("Failed to insert test faces");

    let mut repo = PgFaceRepository::new(pool);

    let filters = FaceFindFilters {
        photo_id: Some(photos[0].id),
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
        .expect("Failed to find faces by photo_id");

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].photo_id, photos[0].id);
    assert_eq!(result.items[0].bbox_x, 100);
}
