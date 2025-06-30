use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct PhotoFindFilters {
    pub text_embedding: Option<Vec<f32>>,
    pub threshold: Option<f32>,

    pub country_id: Option<i32>,
    pub city_id: Option<i32>,

    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}
