#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use dotenv::dotenv;

mod schema;
mod cities;
mod connection;

fn main() {
    dotenv().ok();
    cities::router::create_routes();
}
