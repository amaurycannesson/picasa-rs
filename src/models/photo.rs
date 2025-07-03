use crate::{database::schema::photos, models::pagination::PaginatedResult};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName, Selectable};
use pgvector::Vector;
use postgis_diesel::types::Point;

#[derive(Debug, Queryable, Selectable, Insertable, Default, Clone, QueryableByName)]
#[diesel(table_name = photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    pub id: i32,
    pub path: String,
    pub file_name: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
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
    pub face_detection_completed: bool,
    pub country_id: Option<i32>,
    pub city_id: Option<i32>,
    pub indexed_at: DateTime<Utc>,
}

#[derive(AsChangeset, Debug, Default)]
#[diesel(table_name = photos)]
pub struct UpdatedPhoto {
    pub embedding: Option<Option<Vector>>,
    pub face_detection_completed: Option<bool>,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = photos)]
pub struct PhotoPath {
    pub id: i32,
    pub path: String,
}

pub type PaginatedPhotos = PaginatedResult<Photo>;

pub type PaginatedPhotoPaths = PaginatedResult<PhotoPath>;
