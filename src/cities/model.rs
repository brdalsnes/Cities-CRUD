
use crate::schema::cities;
use serde_derive::{Serialize, Deserialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "cities"]
pub struct City {
    pub id: i32,
    pub name: String,
    pub population: i32,
    pub country: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "cities"]
pub struct InsertableCity {
    pub name: String,
    pub population: i32,
    pub country: String,
}