use picasa_core::models;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CityName {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CountryName {
    pub id: i32,
    pub name: Option<String>,
}

impl From<models::CityName> for CityName {
    fn from(city_name: models::CityName) -> Self {
        Self {
            id: city_name.id,
            name: city_name.name,
        }
    }
}

impl From<models::CountryName> for CountryName {
    fn from(country_name: models::CountryName) -> Self {
        Self {
            id: country_name.id,
            name: country_name.name,
        }
    }
}
