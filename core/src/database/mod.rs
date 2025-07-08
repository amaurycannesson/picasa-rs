use anyhow::{Context, Result};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::env;

pub mod schema;
pub mod sql_functions;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Creates a connection pool for the PostgreSQL database.
pub fn create_pool() -> Result<DbPool> {
    let database_url = build_database_url()?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(15)
        .build(manager)
        .context("Failed to create connection pool")?;

    Ok(pool)
}

/// Runs pending migrations using a connection from the pool.
pub fn run_migrations(pool: &DbPool) {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

/// Builds the database URL from environment variables.
fn build_database_url() -> Result<String> {
    let db_name = env::var("PICASA_POSTGRES_DB").context("PICASA_POSTGRES_DB must be set")?;
    let user = env::var("PICASA_POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("PICASA_POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let host = env::var("PICASA_POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PICASA_POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    );

    Ok(db_url)
}
