use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::cities)]
pub struct City {
    pub geonameid: i32,
    pub name: String,
    pub asciiname: Option<String>,
    pub alternatenames: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
    pub feature_class: Option<String>,
    pub feature_code: Option<String>,
    pub country_code: Option<String>,
    pub cc2: Option<String>,
    pub admin1_code: Option<String>,
    pub admin2_code: Option<String>,
    pub admin3_code: Option<String>,
    pub admin4_code: Option<String>,
    pub population: Option<i32>,
    pub elevation: Option<i32>,
    pub dem: Option<i32>,
    pub timezone: Option<String>,
    pub modification_date: Option<String>,
    pub geom: Option<postgis_diesel::types::Point>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::cities)]
pub struct CityName {
    #[diesel(column_name = geonameid)]
    pub id: i32,
    pub name: String,
}
