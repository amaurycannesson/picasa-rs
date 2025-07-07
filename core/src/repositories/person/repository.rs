use anyhow::{Context, Error, Result};
use diesel::{RunQueryDsl, SelectableHelper};

use crate::{
    database::{DbConnection, DbPool, schema},
    models::{NewPerson, Person},
};

#[cfg_attr(test, mockall::automock)]
pub trait PersonRepository {
    /// Inserts a person and returns the created person.
    fn insert_one(&mut self, new_person: NewPerson) -> Result<Person>;
}

pub struct PgPersonRepository {
    pool: DbPool,
}

impl PgPersonRepository {
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

impl PersonRepository for PgPersonRepository {
    fn insert_one(&mut self, new_person: NewPerson) -> Result<Person> {
        let mut conn = self.get_connection()?;

        let person = diesel::insert_into(schema::people::table)
            .values(&new_person)
            .returning(Person::as_returning())
            .get_result(&mut conn)?;

        Ok(person)
    }
}
