use diesel::RunQueryDsl;
use diesel::{QueryDsl, SelectableHelper, connection::SimpleConnection};
use picasa_core::{
    database::{DbPool, schema},
    models::Photo,
};
use std::fs;

pub fn load_photos(pool: DbPool) -> Vec<Photo> {
    let mut conn = pool.get().unwrap();

    schema::photos::table
        .select(Photo::as_select())
        .load(&mut conn)
        .expect("Failed to load photos")
}

pub fn insert_photo_fixtures(pool: DbPool) {
    let mut conn = pool.get().unwrap();

    let sql =
        fs::read_to_string("tests/data/fixtures/photos.sql").expect("Failed to read SQL file");
    conn.batch_execute(&sql)
        .expect("Failed to execute SQL script");
}
