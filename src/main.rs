#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod domains;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket"
}

fn main() {
    rocket::ignite()
        .mount("/bugs", routes![domains::bug::find_all_bugs])
        .launch();
}
