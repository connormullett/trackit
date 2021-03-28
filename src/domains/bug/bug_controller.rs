use crate::connection::DbConn;

use super::{bug_service, BugDto, NewBug};
use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_all_bugs(connection: DbConn) -> Result<Json<Vec<BugDto>>, Status> {
    Ok(bug_service::get_all_bugs(connection).map(|bugs| Json(bugs))?)
}

#[post("/", data = "<bug>")]
pub fn create_bug(
    bug: Json<NewBug>,
    conn: DbConn,
) -> Result<status::Created<Json<BugDto>>, Status> {
    bug_service::create_bug(bug.into_inner(), conn)
}
