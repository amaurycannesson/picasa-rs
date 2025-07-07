use diesel::prelude::*;
use diesel::sql_types::*;

#[derive(QueryableByName, Debug)]
pub struct FaceCluster {
    #[diesel(sql_type = Integer)]
    pub cluster_id: i32,
    #[diesel(sql_type = Integer)]
    pub representative_face_id: i32,
    #[diesel(sql_type = Integer)]
    pub face_count: i32,
    #[diesel(sql_type = Array<Integer>)]
    pub face_ids: Vec<i32>,
    #[diesel(sql_type = Array<Text>)]
    pub photo_paths: Vec<String>,
    #[diesel(sql_type = Array<Integer>)]
    pub face_ids_without_person: Vec<i32>,
    #[diesel(sql_type = Array<Integer>)]
    pub face_ids_with_person: Vec<i32>,
    #[diesel(sql_type = Array<Integer>)]
    pub person_ids: Vec<i32>,
    #[diesel(sql_type = Float4)]
    pub avg_similarity_score: f32,
    #[diesel(sql_type = Float4)]
    pub min_similarity_score: f32,
}
