use crate::database::{DbConnection, DbPool, sql_functions};
use anyhow::{Context, Error, Result};

use diesel::RunQueryDsl;
use mockall::automock;

#[automock]
pub trait GeoRepository {
    fn find_country_id_by_name(&mut self, name: String) -> Result<Option<i32>>;

    fn find_city_id_by_name(&mut self, name: String) -> Result<Option<i32>>;
}

pub struct PgGeoRepository {
    pool: DbPool,
}

impl PgGeoRepository {
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

impl GeoRepository for PgGeoRepository {
    fn find_country_id_by_name(&mut self, name: String) -> Result<Option<i32>> {
        let mut conn = self.get_connection()?;
        let result: Option<i32> =
            diesel::select(sql_functions::find_country_id_by_name(name)).get_result(&mut conn)?;
        Ok(result)
    }

    fn find_city_id_by_name(&mut self, name: String) -> Result<Option<i32>> {
        let mut conn = self.get_connection()?;
        let result: Option<i32> =
            diesel::select(sql_functions::find_country_id_by_name(name)).get_result(&mut conn)?;
        Ok(result)
    }
}
