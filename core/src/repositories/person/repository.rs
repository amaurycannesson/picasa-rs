use anyhow::{Context, Error, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    database::{DbConnection, DbPool, schema},
    models::{NewPerson, Person},
    repositories::FindPersonFilters,
};

#[cfg_attr(test, mockall::automock)]
pub trait PersonRepository {
    /// Inserts a person and returns the created person.
    fn insert_one(&mut self, new_person: NewPerson) -> Result<Person>;

    /// Retrieves persons with optional filters.
    fn find_many(&mut self, filters: FindPersonFilters) -> Result<Vec<Person>>;

    /// Finds a person by ID.
    fn find_by_id(&mut self, id: i32) -> Result<Person>;
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

    fn find_many(&mut self, filters: FindPersonFilters) -> Result<Vec<Person>> {
        let mut conn = self.get_connection()?;

        let mut query =
            diesel::QueryDsl::select(schema::people::table, Person::as_select()).into_boxed();

        if let Some(ids) = filters.ids {
            query = query.filter(schema::people::id.eq_any(ids));
        }

        let people = query.load(&mut conn)?;

        Ok(people)
    }

    fn find_by_id(&mut self, id: i32) -> Result<Person> {
        let mut conn = self.get_connection()?;

        let person = diesel::QueryDsl::select(schema::people::table, Person::as_select())
            .find(id)
            .get_result(&mut conn)?;

        Ok(person)
    }
}
