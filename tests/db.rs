use std::sync::Once;

use diesel::RunQueryDsl;
use picasa_rs::database::{self, DbPool, schema};

static INIT: Once = Once::new();

pub fn get_pool() -> DbPool {
    dotenvy::from_filename_override(".env.test").ok();

    let mut pool = database::create_pool();
    let mut conn = pool.get().unwrap();

    INIT.call_once(|| {
        database::run_migrations(&mut pool);
    });

    diesel::delete(schema::photos::table)
        .execute(&mut conn)
        .expect("Failed to clean test data");

    pool
}
