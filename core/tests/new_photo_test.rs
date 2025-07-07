use std::path::Path;

use picasa_core::{
    models::new_photo::NewPhoto,
    utils::{compute_file_hash, extract_exif},
};

#[test]
fn test_should_create_new_photo() {
    let path = Path::new("tests/data/images/sunrise_laos.heic");

    let mut new_photo = NewPhoto::new(path).unwrap();
    new_photo = new_photo.with_exif(extract_exif(path).unwrap());
    new_photo = new_photo.with_hash(compute_file_hash(path).unwrap());

    assert_eq!(
        format!("{:?}", new_photo.date_taken_local),
        "Some(2025-02-22T06:55:25)"
    );
    assert_eq!(
        format!("{:?}", new_photo.date_taken_utc),
        "Some(2025-02-21T23:55:25Z)"
    );
    assert_eq!(new_photo.image_width, Some(4032));
    assert_eq!(new_photo.image_height, Some(3024));
}
