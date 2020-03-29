use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status;
use diesel::result::Error;

use crate::connection::DbConn;
use crate::cities::repository;
use crate::cities::model::{City, InsertableCity};

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<City>>, Status> {
    repository::all(&connection)
        .map(|people| Json(people))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<City>, Status> {
    repository::get(id, &connection)
        .map(|city| Json(city))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<city>")]
pub fn post(city: Json<InsertableCity>, connection: DbConn) -> Result<status::Created<Json<City>>, Status> {
    repository::insert(city.into_inner(), &connection)
        .map(|city| status::Created("my_service".to_string(), Some(Json(city))))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<city>")]
pub fn put(id: i32, city: Json<City>, connection: DbConn) -> Result<Json<City>, Status> {
    repository::update(id, city.into_inner(), &connection)
        .map(|city| Json(city))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match repository::get(id, &connection) {
        Ok(_) => repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}