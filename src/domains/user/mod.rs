pub mod user_controller;
pub mod user_dto;
pub mod user_model;
pub mod user_repository;
pub mod user_service;

use rocket::Rocket;

pub use self::user_controller::*;
pub use self::user_dto::*;
pub use self::user_model::*;
pub use self::user_repository::*;
pub use self::user_service::*;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/users", routes![user_controller::register])
}
