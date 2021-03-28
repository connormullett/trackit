use diesel::result::Error;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use rocket::{http::Status, response::status};

use super::{bug_repository, NewBug};
use super::{BugDto, BugModel};

pub fn get_all_bugs(conn: DbConn) -> Result<Vec<BugDto>, Status> {
    bug_repository::get_all_bugs(&conn)
        .map(|bugs| {
            bugs.iter()
                .map(|bug| BugDto {
                    id: bug.id,
                    title: bug.title.clone(),
                    body: bug.body.clone(),
                    resolved: bug.resolved,
                })
                .collect()
        })
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

pub fn create_bug(bug: NewBug, conn: DbConn) -> Result<status::Created<Json<BugDto>>, Status> {
    bug_repository::create_bug(bug, &conn)
        .map(|person| person_created(person))
        .map_err(|error| error_status(error))
}

fn person_created(bug: BugModel) -> status::Created<Json<BugDto>> {
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
        Some(Json(BugDto {
            id: bug.id,
            title: bug.title,
            body: bug.body,
            resolved: bug.resolved,
        })),
    )
}
