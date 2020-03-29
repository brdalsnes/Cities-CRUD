use crate::diesel::{RunQueryDsl, QueryDsl};
use diesel::{PgConnection, QueryResult};
use crate::schema::cities;
use crate::cities::model::{City, InsertableCity};

pub fn all(connection: &PgConnection) -> QueryResult<Vec<City>> {
    cities::table.load::<City>(connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<City> {
    cities::table.find(id).get_result::<City>(connection)
}

pub fn insert(city: InsertableCity, connection: &PgConnection) -> QueryResult<City> {
    diesel::insert_into(cities::table)
        .values(&city)
        .get_result(connection)
}

pub fn update(id: i32, city: City, connection: &PgConnection) -> QueryResult<City> {
    diesel::update(cities::table.find(id))
        .set(&city)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(cities::table.find(id))
        .execute(connection)
}