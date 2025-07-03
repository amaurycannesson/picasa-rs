use crate::{
    database::{DbConnection, DbPool, schema},
    models::{Face, NewFace, PaginatedFaces, PaginationFilter},
    repositories::face::filters::FaceFindFilters,
    utils::serialize_float_array,
};
use anyhow::{Context, Error, Result};
use diesel::{
    dsl::sql,
    prelude::*,
    sql_types::{Bool, Float},
};
use mockall::automock;

#[automock]
pub trait FaceRepository {
    /// Inserts a single face and returns the created face.
    fn insert_one(&mut self, new_face: NewFace) -> Result<Face>;

    /// Find faces with filters, pagination, and sorting
    fn find(
        &mut self,
        pagination: PaginationFilter,
        filters: FaceFindFilters,
    ) -> Result<PaginatedFaces>;
}

pub struct PgFaceRepository {
    pool: DbPool,
}

impl PgFaceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_connection(&self) -> Result<DbConnection, Error> {
        self.pool
            .get()
            .map_err(Error::from)
            .context("Failed to get database connection")
    }

    fn build_semantic_filter_sql(face_embedding: &[f32], threshold: f32) -> String {
        format!(
            "(1 - (faces.embedding <=> '{}'::vector)) > {}",
            serialize_float_array(face_embedding),
            threshold
        )
    }

    fn build_semantic_order_sql(face_embedding: &[f32]) -> String {
        format!(
            "1 - (faces.embedding <=> '{}'::vector)",
            serialize_float_array(face_embedding)
        )
    }
}

impl FaceRepository for PgFaceRepository {
    fn insert_one(&mut self, new_face: NewFace) -> Result<Face> {
        let mut conn = self.get_connection()?;

        let face = diesel::insert_into(schema::faces::table)
            .values(&new_face)
            .returning(Face::as_returning())
            .get_result(&mut conn)?;

        Ok(face)
    }

    fn find(
        &mut self,
        pagination: PaginationFilter,
        filters: FaceFindFilters,
    ) -> Result<PaginatedFaces> {
        let mut conn = self.get_connection()?;
        let offset = (pagination.page - 1) * pagination.per_page;

        // Build base count query
        let mut count_query = schema::faces::table
            .select(diesel::dsl::count_star())
            .into_boxed();

        // Build base select query
        let mut select_query = schema::faces::table.select(Face::as_select()).into_boxed();

        // Apply semantic filter if present
        if let Some(ref face_embedding) = filters.face_embedding {
            let threshold = filters.threshold.unwrap_or(0.0);
            let semantic_filter_sql = Self::build_semantic_filter_sql(face_embedding, threshold);

            count_query = count_query.filter(sql::<Bool>(&semantic_filter_sql));
            select_query = select_query.filter(sql::<Bool>(&semantic_filter_sql));

            // Add semantic ordering
            let order_sql = Self::build_semantic_order_sql(face_embedding);
            select_query = select_query.order(sql::<Float>(&order_sql).desc());
        }

        // Apply photo_id filter if present
        if let Some(photo_id) = filters.photo_id {
            count_query = count_query.filter(schema::faces::photo_id.eq(photo_id));
            select_query = select_query.filter(schema::faces::photo_id.eq(photo_id));
        }

        // Apply person_id filter if present
        if let Some(person_id) = filters.person_id {
            count_query = count_query.filter(schema::faces::person_id.eq(person_id));
            select_query = select_query.filter(schema::faces::person_id.eq(person_id));
        }

        // Get total count of filtered faces
        let total: i64 = count_query.first(&mut conn)?;

        // Get paginated faces with consistent ordering
        let faces = select_query
            .then_order_by(schema::faces::id.asc())
            .limit(pagination.per_page)
            .offset(offset)
            .load(&mut conn)?;

        // Calculate total pages (ceiling division)
        let total_pages = (total + pagination.per_page - 1) / pagination.per_page;

        Ok(PaginatedFaces {
            items: faces,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }
}
