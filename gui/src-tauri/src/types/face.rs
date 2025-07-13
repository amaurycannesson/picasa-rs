use picasa_core::models;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Face {
    pub id: i32,
    pub photo_id: i32,
    pub bbox_x: i32,
    pub bbox_y: i32,
    pub bbox_width: i32,
    pub bbox_height: i32,
    pub confidence: f32,
    pub gender: Option<String>,
    pub person_id: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<models::Face> for Face {
    fn from(core_face: models::Face) -> Self {
        Self {
            id: core_face.id,
            photo_id: core_face.photo_id,
            bbox_x: core_face.bbox_x,
            bbox_y: core_face.bbox_y,
            bbox_width: core_face.bbox_width,
            bbox_height: core_face.bbox_height,
            confidence: core_face.confidence,
            gender: core_face.gender,
            person_id: core_face.person_id,
            created_at: core_face.created_at.to_rfc3339(),
            updated_at: core_face.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct PaginatedFaces {
    pub items: Vec<Face>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl From<models::PaginatedFaces> for PaginatedFaces {
    fn from(paginated_faces: models::PaginatedFaces) -> Self {
        Self {
            items: paginated_faces
                .items
                .into_iter()
                .map(Face::from)
                .collect(),
            total: paginated_faces.total,
            page: paginated_faces.page,
            per_page: paginated_faces.per_page,
            total_pages: paginated_faces.total_pages,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct PendingFaceReview {
    pub cluster_id: i32,
    pub face_ids: Vec<i32>,
    pub confidence: f32,
    pub face_count: i32,
}
