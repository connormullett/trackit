#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod connection;
mod domains;
pub mod schema;

use dotenv;

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .mount(
            "/bugs",
            routes![
                domains::bug::get_all_bugs,
                domains::bug::create_bug,
                domains::bug::get_bug_by_id,
                domains::bug::update_bug,
                domains::bug::delete_bug
            ],
        )
        .manage(connection::init_pool())
        .launch();
}
