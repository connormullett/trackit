#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod domains;
pub mod schema;

use dotenv;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket"
}

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .mount("/bugs", routes![domains::bug::find_all_bugs])
        .launch();
}
