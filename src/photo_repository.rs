use crate::{
    database::{DbPool, schema},
    models::{
        geospatial_search_result::GeospatialSearchResult,
        photo::{PaginatedPaths, Photo, PhotoEmbedding},
        semantic_search_result::SemanticSearchResult,
    },
};
use diesel::{prelude::*, sql_query};
use mockall::automock;

#[automock]
pub trait PhotoRepository {
    /// Lists photo paths that do not have embeddings, with pagination.
    fn list_paths_without_embedding(
        &mut self,
        page: i64,
        per_page: i64,
    ) -> QueryResult<PaginatedPaths>;

    /// Inserts a batch of new photos.
    fn insert_batch(&mut self, new_photos: Vec<Photo>) -> QueryResult<usize>;

    /// Updates embeddings for a batch of photos.
    fn update_embeddings(&mut self, embeddings: Vec<PhotoEmbedding>) -> QueryResult<usize>;

    /// Find by text query.
    fn find_by_text(
        &mut self,
        text_embedding: Vec<f32>,
        threshold: Option<f32>,
        limit: Option<usize>,
    ) -> QueryResult<Vec<SemanticSearchResult>>;

    /// Find by country.
    fn find_by_country(&mut self, country_query: &str) -> QueryResult<Vec<GeospatialSearchResult>>;
}

pub struct PgPhotoRepository {
    pool: DbPool,
}

impl PgPhotoRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PhotoRepository for PgPhotoRepository {
    fn list_paths_without_embedding(
        &mut self,
        page: i64,
        per_page: i64,
    ) -> QueryResult<PaginatedPaths> {
        let mut conn = self.pool.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;
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

    fn insert_batch(&mut self, new_photos: Vec<Photo>) -> QueryResult<usize> {
        let mut conn = self.pool.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;
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
                schema::photos::date_taken.eq(excluded(schema::photos::date_taken)),
                schema::photos::gps_location.eq(excluded(schema::photos::gps_location)),
                schema::photos::image_width.eq(excluded(schema::photos::image_width)),
                schema::photos::image_height.eq(excluded(schema::photos::image_height)),
            ))
            .execute(&mut conn)
    }

    fn update_embeddings(&mut self, embeddings: Vec<PhotoEmbedding>) -> QueryResult<usize> {
        let mut conn = self.pool.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;
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

    fn find_by_text(
        &mut self,
        text_embedding: Vec<f32>,
        threshold: Option<f32>,
        limit: Option<usize>,
    ) -> QueryResult<Vec<SemanticSearchResult>> {
        let mut conn = self.pool.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;
        let embedding_str = format!(
            "[{}]",
            text_embedding
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        sql_query("SELECT * FROM similarity_search($1::vector, $2, $3)")
            .bind::<diesel::sql_types::Text, _>(embedding_str)
            .bind::<diesel::sql_types::Float, _>(threshold.unwrap_or(0.0))
            .bind::<diesel::sql_types::Integer, _>(limit.unwrap_or(10) as i32)
            .get_results::<SemanticSearchResult>(&mut conn)
    }

    fn find_by_country(&mut self, country_query: &str) -> QueryResult<Vec<GeospatialSearchResult>> {
        let mut conn = self.pool.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;
        sql_query("SELECT * FROM find_photos_by_country($1)")
            .bind::<diesel::sql_types::Text, _>(country_query)
            .get_results::<GeospatialSearchResult>(&mut conn)
    }
}
