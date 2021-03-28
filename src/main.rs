#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod connection;
mod domains;
pub mod schema;

use domains::bug;
use dotenv;

fn main() {
    dotenv::dotenv().ok();

    let mut rocket = rocket::ignite().manage(connection::init_pool());

    rocket = bug::mount(rocket);

    rocket.launch();
}
