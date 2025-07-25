use crate::config::DatabaseConfig;
use anyhow::{Context, Result};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub mod schema;
pub mod sql_functions;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Creates a connection pool using the provided database configuration.
pub fn create_pool(db_config: &DatabaseConfig) -> Result<DbPool> {
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.database
    );
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(db_config.max_connections)
        .build(manager)
        .context("Failed to create connection pool")?;

    Ok(pool)
}

/// Runs pending migrations using a connection from the pool.
pub fn run_migrations(pool: &DbPool) {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
