use crate::{database::schema::photos, models::pagination::PaginatedResult};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, QueryableByName, Selectable};
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
    pub country_id: Option<i32>,
    pub city_id: Option<i32>,
}

#[derive(Debug)]
pub struct PaginatedPaths {
    pub paths: Vec<String>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug)]
pub struct PhotoEmbedding {
    pub path: String,
    pub embedding: Option<Vector>,
}

pub type PaginatedPhotos = PaginatedResult<Photo>;
