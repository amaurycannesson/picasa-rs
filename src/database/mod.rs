use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::env;

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Runs pending migrations on the provided database connection.
pub fn run_migrations(connection: &mut PgConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

/// Establishes a connection to the PostgreSQL database using individual PostgreSQL environment variables.
pub fn establish_connection() -> PgConnection {
    let db_name = env::var("PICASA_POSTGRES_DB").expect("PICASA_POSTGRES_DB must be set");
    let user = env::var("PICASA_POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("PICASA_POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let host = env::var("PICASA_POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PICASA_POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());

    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    );

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
