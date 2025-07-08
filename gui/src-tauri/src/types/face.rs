use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct PendingFaceReview {
    pub cluster_id: i32,
    pub face_ids: Vec<i32>,
    pub confidence: f32,
    pub face_count: i32,
}
