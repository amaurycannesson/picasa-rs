use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum PersonMatchMode {
    Any,
    All,
}

impl Default for PersonMatchMode {
    fn default() -> Self {
        PersonMatchMode::Any
    }
}

#[derive(Debug, Clone, Default)]
pub struct PhotoFindFilters {
    pub text_embedding: Option<Vec<f32>>,
    pub threshold: Option<f32>,

    pub country_id: Option<i32>,
    pub city_id: Option<i32>,

    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,

    pub person_ids: Option<Vec<i32>>,
    pub person_match_mode: Option<PersonMatchMode>,
}

#[derive(Debug, Clone, Default)]
pub struct PhotoFindPathFilters {
    pub has_face_detection_completed: Option<bool>,
    pub has_embedding: Option<bool>,
}
