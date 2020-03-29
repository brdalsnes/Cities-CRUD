use rocket;

use crate::connection;
use crate::cities::handler;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/cities",
               routes![
                   handler::all,
                   handler::get,
                   handler::post,
                   handler::put,
                   handler::delete
               ],
        ).launch();
}