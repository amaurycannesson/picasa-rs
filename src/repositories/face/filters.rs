#[derive(Debug, Clone, Default)]
pub struct FaceFindFilters {
    pub photo_id: Option<i32>,
    pub person_id: Option<i32>,
    pub face_embedding: Option<Vec<f32>>,
    pub threshold: Option<f32>,
    pub has_embedding: Option<bool>,
    pub has_person: Option<bool>,
}
