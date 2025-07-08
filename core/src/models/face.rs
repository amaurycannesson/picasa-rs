use chrono::{DateTime, Utc};
use diesel::prelude::*;
use pgvector::Vector;

use crate::{database::schema::faces, models::PaginatedResult};

#[derive(Queryable, Selectable, Debug, Default)]
#[diesel(table_name = faces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Face {
    pub id: i32,
    pub photo_id: i32,
    pub bbox_x: i32,
    pub bbox_y: i32,
    pub bbox_width: i32,
    pub bbox_height: i32,
    pub confidence: f32,
    pub gender: Option<String>,
    pub embedding: Option<Vector>,
    pub person_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Default)]
#[diesel(table_name = faces)]
pub struct NewFace {
    pub photo_id: i32,
    pub person_id: Option<i32>,
    pub bbox_x: i32,
    pub bbox_y: i32,
    pub bbox_width: i32,
    pub bbox_height: i32,
    pub confidence: f32,
    pub gender: Option<String>,
    pub embedding: Option<Vector>,
}

#[derive(AsChangeset, Debug, Default)]
#[diesel(table_name = faces)]
pub struct UpdatedFace {
    pub person_id: Option<Option<i32>>,
}

#[derive(Debug)]
pub struct FaceWithPhoto {
    pub face: Face,
    pub photo_id: i32,
    pub photo_path: String,
}

pub type PaginatedFaces = PaginatedResult<Face>;
