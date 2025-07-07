use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::database::schema::countries)]
pub struct Country {
    pub gid: i32,
    pub name: Option<String>,
    pub name_long: Option<String>,
    pub formal_en: Option<String>,
    pub formal_fr: Option<String>,
    pub pop_est: Option<f64>,
    pub pop_rank: Option<i16>,
    pub pop_year: Option<i16>,
    pub gdp_md: Option<i32>,
    pub gdp_year: Option<i16>,
    pub economy: Option<String>,
    pub income_grp: Option<String>,
    pub iso_a2: Option<String>,
    pub iso_a3: Option<String>,
    pub iso_n3: Option<String>,
    pub adm0_a3_id: Option<String>,
    pub continent: Option<String>,
    pub region_un: Option<String>,
    pub subregion: Option<String>,
    pub region_wb: Option<String>,
    pub name_len: Option<i16>,
    pub long_len: Option<i16>,
    pub abbrev_len: Option<i16>,
    pub wikidataid: Option<String>,
    pub geom: Option<postgis_diesel::types::Point>,
}
