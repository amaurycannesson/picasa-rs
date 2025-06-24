use crate::{
    database::schema,
    models::photo::{PaginatedPaths, Photo, PhotoEmbedding},
};
use diesel::prelude::*;
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
}

pub struct PgPhotoRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> PgPhotoRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }
}

impl<'a> PhotoRepository for PgPhotoRepository<'a> {
    fn list_paths_without_embedding(
        &mut self,
        page: i64,
        per_page: i64,
    ) -> QueryResult<PaginatedPaths> {
        let offset = (page - 1) * per_page;

        let total: i64 = schema::photos::table
            .select(diesel::dsl::count_star())
            .filter(schema::photos::embedding.is_null())
            .first(self.conn)?;

        let paths = schema::photos::table
            .select(schema::photos::path)
            .filter(schema::photos::embedding.is_null())
            .limit(per_page)
            .offset(offset)
            .load(self.conn)?;

        Ok(PaginatedPaths {
            paths,
            total,
            page,
            per_page,
        })
    }

    fn insert_batch(&mut self, new_photos: Vec<Photo>) -> QueryResult<usize> {
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
            .execute(self.conn)
    }

    fn update_embeddings(&mut self, embeddings: Vec<PhotoEmbedding>) -> QueryResult<usize> {
        let mut total_updated = 0;
        for embedding in embeddings {
            let updated = diesel::update(schema::photos::table)
                .filter(schema::photos::path.eq(&embedding.path))
                .set(schema::photos::embedding.eq(&embedding.embedding))
                .execute(self.conn)?;
            total_updated += updated;
        }
        Ok(total_updated)
    }
}
