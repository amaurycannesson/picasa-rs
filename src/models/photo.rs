use crate::{
    database::schema::photos,
    utils::{convert_exif_gps_info_to_postgis_point, system_time_to_naive_datetime},
};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use nom_exif::{Exif, ExifTag};
use pgvector::Vector;
use postgis_diesel::types::Point;
use std::path::Path;

#[derive(Debug, Queryable, Selectable, Insertable, Default, Clone)]
#[diesel(table_name = photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    pub path: String,
    pub file_name: String,
    pub file_size: i64,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub indexed_at: NaiveDateTime,
    pub hash: Option<String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub orientation: Option<i32>,
    pub date_taken: Option<NaiveDateTime>,
    pub gps_location: Option<Point>,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
    pub embedding: Option<Vector>,
}

#[derive(Debug)]
pub struct PaginatedPaths {
    pub paths: Vec<String>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PhotoEmbedding {
    pub path: String,
    pub embedding: Option<Vector>,
}

impl Photo {
    pub fn new(path: &Path) -> Self {
        let metadata = path.metadata().unwrap();
        let now = chrono::Local::now().naive_utc();

        Self {
            path: path.to_string_lossy().into_owned(),
            file_name: path.file_name().unwrap().to_string_lossy().into_owned(),
            file_size: metadata.len() as i64,
            created_at: system_time_to_naive_datetime(metadata.created().unwrap()),
            modified_at: system_time_to_naive_datetime(metadata.modified().unwrap()),
            embedding: None,
            indexed_at: now,
            hash: None,
            camera_make: None,
            camera_model: None,
            lens_model: None,
            orientation: None,
            date_taken: None,
            image_width: None,
            image_height: None,
            gps_location: None,
        }
    }

    pub fn with_hash(mut self, hash: String) -> Self {
        self.hash = Some(hash);
        self
    }

    pub fn with_exif(mut self, exif: Exif) -> Self {
        if let Some(make) = exif.get(ExifTag::Make) {
            self.camera_make = make.as_str().map(|s| s.to_string());
        }

        if let Some(model) = exif.get(ExifTag::Model) {
            self.camera_model = model.as_str().map(|s| s.to_string());
        }

        if let Some(lens) = exif.get(ExifTag::LensModel) {
            self.lens_model = lens.as_str().map(|s| s.to_string());
        }

        if let Some(orientation) = exif.get(ExifTag::Orientation) {
            self.orientation = orientation.as_u64().map(|o| o as i32);
        }

        if let Some(date_time) = exif.get(ExifTag::DateTimeOriginal) {
            if let Some(date_str) = date_time.as_str() {
                if let Ok(parsed_date) =
                    chrono::NaiveDateTime::parse_from_str(date_str, "%Y:%m:%d %H:%M:%S")
                {
                    self.date_taken = Some(parsed_date);
                }
            }
        }

        if let Ok(Some(gps_info)) = exif.get_gps_info() {
            self.gps_location = convert_exif_gps_info_to_postgis_point(gps_info);
        }

        if let Some(width) = exif.get(ExifTag::ImageWidth) {
            self.image_width = width.as_u64().map(|w| w as i32);
        }

        if let Some(height) = exif.get(ExifTag::ImageHeight) {
            self.image_height = height.as_u64().map(|h| h as i32);
        }

        self
    }
}
