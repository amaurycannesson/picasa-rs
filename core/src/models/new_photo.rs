use crate::{database::schema::photos, utils::convert_exif_gps_info_to_postgis_point};
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::Insertable;
use nom_exif::{Exif, ExifTag};
use pgvector::Vector;
use postgis_diesel::types::Point;
use std::path::Path;

#[derive(Debug, Insertable, Default, Clone)]
#[diesel(table_name = photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPhoto {
    pub path: String,
    pub file_name: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub indexed_at: DateTime<Utc>,
    pub hash: Option<String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub orientation: Option<i32>,
    pub date_taken_local: Option<NaiveDateTime>,
    pub date_taken_utc: Option<DateTime<Utc>>,
    pub gps_location: Option<Point>,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
    pub embedding: Option<Vector>,
}

impl NewPhoto {
    pub fn new(path: &Path) -> Result<Self> {
        let metadata = path.metadata().context("Failed to read file metadata")?;
        let now = chrono::Local::now().to_utc();

        Ok(Self {
            path: path.to_string_lossy().into_owned(),
            file_name: path.file_name().unwrap().to_string_lossy().into_owned(),
            file_size: metadata.len() as i64,
            created_at: metadata.created().map(DateTime::<Utc>::from)?,
            modified_at: metadata.created().map(DateTime::<Utc>::from)?,
            embedding: None,
            indexed_at: now,
            hash: None,
            camera_make: None,
            camera_model: None,
            lens_model: None,
            orientation: None,
            date_taken_utc: None,
            date_taken_local: None,
            image_width: None,
            image_height: None,
            gps_location: None,
        })
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
            self.orientation = orientation.as_i32();
        }

        if let Some(date_time_original) = exif.get(ExifTag::DateTimeOriginal) {
            self.date_taken_utc = date_time_original.as_time().map(|t| t.to_utc());
            self.date_taken_local = date_time_original.as_time().map(|t| t.naive_local());
        }

        if let Ok(Some(gps_info)) = exif.get_gps_info() {
            self.gps_location = convert_exif_gps_info_to_postgis_point(gps_info);
        }

        if let Some(width) = exif.get(ExifTag::ExifImageWidth) {
            self.image_width = width.as_u32().map(|w| w as i32);
        }

        if let Some(height) = exif.get(ExifTag::ExifImageHeight) {
            self.image_height = height.as_u32().map(|h| h as i32);
        }

        self
    }
}
