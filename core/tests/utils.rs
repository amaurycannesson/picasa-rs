use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, connection::SimpleConnection};
use picasa_core::{
    config::Config,
    database::{self, DbPool, schema},
    models::Photo,
};
use std::fs;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn get_pool() -> DbPool {
    let config = load_config();
    let mut pool = database::create_pool(&config.database).expect("Failed to create databse pool");
    let mut conn = pool.get().unwrap();

    INIT.call_once(|| {
        database::run_migrations(&mut pool);
    });

    diesel::delete(schema::photos::table)
        .execute(&mut conn)
        .expect("Failed to clean test data");

    pool
}

pub fn load_config() -> Config {
    Config::load().expect("Failed to load config")
}

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
