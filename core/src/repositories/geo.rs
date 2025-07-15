use anyhow::{Context, Error, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    database::{DbConnection, DbPool, schema, sql_functions},
    models::{CityName, CountryName},
};

#[cfg_attr(test, mockall::automock)]
pub trait GeoRepository {
    /// Finds a country ID by its name.
    fn find_country_id_by_name(&mut self, name: String) -> Result<Option<i32>>;

    /// Finds a city ID by its name.
    fn find_city_id_by_name(&mut self, name: String) -> Result<Option<i32>>;

    /// Finds country names by their IDs.
    fn find_country_names_by_ids(&mut self, ids: Vec<i32>) -> Result<Vec<CountryName>>;

    /// Finds city names by their IDs.
    fn find_city_names_by_ids(&mut self, ids: Vec<i32>) -> Result<Vec<CityName>>;
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
            diesel::select(sql_functions::find_city_id_by_name(name)).get_result(&mut conn)?;
        Ok(result)
    }

    fn find_country_names_by_ids(&mut self, ids: Vec<i32>) -> Result<Vec<CountryName>> {
        let mut conn = self.get_connection()?;

        let countries = schema::countries::table
            .select(CountryName::as_select())
            .filter(schema::countries::gid.eq_any(ids))
            .order_by(schema::countries::name)
            .load(&mut conn)?;

        Ok(countries)
    }

    fn find_city_names_by_ids(&mut self, ids: Vec<i32>) -> Result<Vec<CityName>> {
        let mut conn = self.get_connection()?;

        let cities = schema::cities::table
            .select((
                schema::cities::geonameid,
                sql_functions::coalesce(schema::cities::asciiname, schema::cities::name),
            ))
            .filter(schema::cities::geonameid.eq_any(ids))
            .order_by(sql_functions::coalesce(
                schema::cities::asciiname,
                schema::cities::name,
            ))
            .load(&mut conn)?;

        Ok(cities)
    }
}
