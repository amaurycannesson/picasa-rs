use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::database::schema::people;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = people)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = people)]
pub struct NewPerson {
    pub name: String,
}
