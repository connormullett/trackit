pub mod auth_controller;
pub mod auth_service;
pub mod dto;

use rocket::Rocket;

pub use self::auth_controller::*;
pub use self::auth_service::*;
pub use self::dto::*;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/auth", routes![auth_controller::login])
}
