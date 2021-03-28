use diesel::result::Error;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use rocket::{http::Status, response::status};

use super::{bug_repository, NewBug};
use super::{BugDto, BugModel};

pub fn get_all_bugs(conn: DbConn) -> Result<Vec<BugDto>, Status> {
    bug_repository::get_all_bugs(&conn)
        .map(|bugs| {
            bugs.into_iter()
                .map(|bug| BugDto::from_model(bug))
                .collect()
        })
        .map_err(|error| error_status(error))
}

pub fn create_bug(bug: NewBug, conn: DbConn) -> Result<status::Created<Json<BugDto>>, Status> {
    bug_repository::create_bug(bug, &conn)
        .map(|bug| bug_created(bug))
        .map_err(|error| error_status(error))
}

pub fn get_bug_by_id(id: i32, conn: DbConn) -> Result<BugDto, Status> {
    bug_repository::get_bug_by_id(id, &conn)
        .map(|bug| BugDto::from_model(bug))
        .map_err(|error| error_status(error))
}

pub fn update_bug(id: i32, bug: BugDto, conn: DbConn) -> Result<BugDto, Status> {
    bug_repository::update_bug(id, bug, &conn)
        .map(|bug| BugDto::from_model(bug))
        .map_err(|error| error_status(error))
}

pub fn delete_bug(id: i32, conn: DbConn) -> Result<status::NoContent, Status> {
    match bug_repository::get_bug_by_id(id, &conn) {
        Ok(_) => bug_repository::delete_bug(id, &conn)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(err) => Err(error_status(err)),
    }
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn bug_created(bug: BugModel) -> status::Created<Json<BugDto>> {
    let host = dotenv::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = dotenv::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!(
            "{host}:{port}/people/{id}",
            host = host,
            port = port,
            id = bug.id
        )
        .to_string(),
        Some(Json(BugDto::from_model(bug))),
    )
}
