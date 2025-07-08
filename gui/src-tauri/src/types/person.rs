use picasa_core::models;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

impl From<models::Person> for Person {
    fn from(core_person: models::Person) -> Self {
        Self {
            id: core_person.id,
            name: core_person.name,
        }
    }
}
