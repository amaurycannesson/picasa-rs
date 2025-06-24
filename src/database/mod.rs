use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::env;

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Runs pending migrations on the provided database connection.
pub fn run_migrations(connection: &mut PgConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

/// Establishes a connection to the PostgreSQL database using the `DATABASE_URL` environment variable.
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
