use crate::{
    database::{DbConnection, DbPool, schema},
    models::{NewPhoto, PaginatedPaths, PaginatedPhotos, PaginationFilter, Photo, PhotoEmbedding},
    repositories::PhotoFindFilters,
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
pub trait PhotoRepository {
    /// Lists photo paths that do not have embeddings, with pagination.
    fn list_paths_without_embedding(&mut self, page: i64, per_page: i64) -> Result<PaginatedPaths>;

    /// Inserts a batch of new photos.
    fn insert_batch(&mut self, new_photos: Vec<NewPhoto>) -> Result<usize>;

    /// Updates embeddings for a batch of photos.
    fn update_embeddings(&mut self, embeddings: Vec<PhotoEmbedding>) -> Result<usize>;

    /// Find photos with filters, pagination, and sorting
    fn find(
        &mut self,
        pagination: PaginationFilter,
        filters: PhotoFindFilters,
    ) -> Result<PaginatedPhotos>;
}

pub struct PgPhotoRepository {
    pool: DbPool,
}

impl PgPhotoRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_connection(&self) -> Result<DbConnection, Error> {
        self.pool
            .get()
            .map_err(Error::from)
            .context("Failed to get database connection")
    }
}

impl PgPhotoRepository {
    fn build_semantic_filter_sql(text_embedding: &[f32], threshold: f32) -> String {
        format!(
            "(1 - (photos.embedding <=> '{}'::vector)) > {}",
            serialize_float_array(text_embedding),
            threshold
        )
    }

    fn build_semantic_order_sql(text_embedding: &[f32]) -> String {
        format!(
            "1 - (photos.embedding <=> '{}'::vector)",
            serialize_float_array(text_embedding)
        )
    }
}

impl PhotoRepository for PgPhotoRepository {
    fn find(
        &mut self,
        pagination: PaginationFilter,
        filters: PhotoFindFilters,
    ) -> Result<PaginatedPhotos> {
        let mut conn = self.get_connection()?;
        let offset = (pagination.page - 1) * pagination.per_page;

        // Build base count query
        let mut count_query = schema::photos::table
            .select(diesel::dsl::count_star())
            .into_boxed();

        // Build base select query
        let mut select_query = schema::photos::table
            .select(Photo::as_select())
            .into_boxed();

        // Apply semantic filter if present
        if let Some(ref text_embedding) = filters.text_embedding {
            let threshold = filters.threshold.unwrap_or(0.0);
            let semantic_filter_sql = Self::build_semantic_filter_sql(text_embedding, threshold);

            count_query = count_query.filter(sql::<Bool>(&semantic_filter_sql));
            select_query = select_query.filter(sql::<Bool>(&semantic_filter_sql));

            // Add semantic ordering
            let order_sql = Self::build_semantic_order_sql(text_embedding);
            select_query = select_query.order(sql::<Float>(&order_sql).desc());
        }

        // Apply country filter if present
        if let Some(country_id) = filters.country_id {
            count_query = count_query.filter(schema::photos::country_id.eq(country_id));
            select_query = select_query.filter(schema::photos::country_id.eq(country_id));
        }

        // Apply city filter if present
        if let Some(city_id) = filters.city_id {
            count_query = count_query.filter(schema::photos::city_id.eq(city_id));
            select_query = select_query.filter(schema::photos::city_id.eq(city_id));
        }

        // Apply date filters
        if let Some(date_from) = filters.date_from {
            count_query = count_query.filter(schema::photos::date_taken_utc.ge(date_from));
            select_query = select_query.filter(schema::photos::date_taken_utc.ge(date_from));
        }

        if let Some(date_to) = filters.date_to {
            count_query = count_query.filter(schema::photos::date_taken_utc.le(date_to));
            select_query = select_query.filter(schema::photos::date_taken_utc.le(date_to));
        }

        // Get total count of filtered photos
        let total: i64 = count_query.first(&mut conn)?;

        // Get paginated photos with consistent ordering
        let photos = select_query
            .then_order_by(schema::photos::id.asc())
            .limit(pagination.per_page)
            .offset(offset)
            .load(&mut conn)?;

        // Calculate total pages (ceiling division)
        let total_pages = (total + pagination.per_page - 1) / pagination.per_page;

        Ok(PaginatedPhotos {
            photos,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    fn list_paths_without_embedding(&mut self, page: i64, per_page: i64) -> Result<PaginatedPaths> {
        let mut conn = self.get_connection()?;
        let offset = (page - 1) * per_page;

        let total: i64 = schema::photos::table
            .select(diesel::dsl::count_star())
            .filter(schema::photos::embedding.is_null())
            .first(&mut conn)?;

        let paths = schema::photos::table
            .select(schema::photos::path)
            .filter(schema::photos::embedding.is_null())
            .limit(per_page)
            .offset(offset)
            .load(&mut conn)?;

        Ok(PaginatedPaths {
            paths,
            total,
            page,
            per_page,
        })
    }

    fn insert_batch(&mut self, new_photos: Vec<NewPhoto>) -> Result<usize> {
        let mut conn = self.get_connection()?;
        use diesel::upsert::excluded;

        diesel::insert_into(schema::photos::table)
            .values(&new_photos)
            .on_conflict(schema::photos::path)
            .do_update()
            .set((
                schema::photos::file_size.eq(excluded(schema::photos::file_size)),
                schema::photos::created_at.eq(excluded(schema::photos::created_at)),
                schema::photos::modified_at.eq(excluded(schema::photos::modified_at)),
                schema::photos::indexed_at.eq(excluded(schema::photos::indexed_at)),
                schema::photos::hash.eq(excluded(schema::photos::hash)),
                schema::photos::camera_make.eq(excluded(schema::photos::camera_make)),
                schema::photos::camera_model.eq(excluded(schema::photos::camera_model)),
                schema::photos::lens_model.eq(excluded(schema::photos::lens_model)),
                schema::photos::orientation.eq(excluded(schema::photos::orientation)),
                schema::photos::date_taken_local.eq(excluded(schema::photos::date_taken_local)),
                schema::photos::date_taken_utc.eq(excluded(schema::photos::date_taken_utc)),
                schema::photos::gps_location.eq(excluded(schema::photos::gps_location)),
                schema::photos::image_width.eq(excluded(schema::photos::image_width)),
                schema::photos::image_height.eq(excluded(schema::photos::image_height)),
            ))
            .execute(&mut conn)
            .map_err(Error::from)
    }

    fn update_embeddings(&mut self, embeddings: Vec<PhotoEmbedding>) -> Result<usize> {
        let mut conn = self.get_connection()?;
        let mut total_updated = 0;
        for embedding in embeddings {
            let updated = diesel::update(schema::photos::table)
                .filter(schema::photos::path.eq(&embedding.path))
                .set(schema::photos::embedding.eq(&embedding.embedding))
                .execute(&mut conn)?;
            total_updated += updated;
        }
        Ok(total_updated)
    }
}
