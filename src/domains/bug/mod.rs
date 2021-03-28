pub mod bug_controller;
pub mod bug_dto;
pub mod bug_model;
pub mod bug_repository;
pub mod bug_service;

use rocket::Rocket;

pub use self::bug_controller::*;
pub use self::bug_dto::*;
pub use self::bug_model::*;
pub use self::bug_repository::*;
pub use self::bug_service::*;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/bugs",
        routes![
            bug_controller::get_all_bugs,
            bug_controller::create_bug,
            bug_controller::get_bug_by_id,
            bug_controller::update_bug,
            bug_controller::delete_bug
        ],
    )
}
