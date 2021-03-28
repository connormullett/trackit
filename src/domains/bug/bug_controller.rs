use crate::connection::DbConn;

use super::{bug_service, BugDto, NewBug};
use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_all_bugs(connection: DbConn) -> Result<Json<Vec<BugDto>>, Status> {
    bug_service::get_all_bugs(connection).map(|bugs| Json(bugs))
}

#[get("/<id>")]
pub fn get_bug_by_id(id: i32, connection: DbConn) -> Result<Json<BugDto>, Status> {
    bug_service::get_bug_by_id(id, connection).map(|bug| Json(bug))
}

#[post("/", data = "<bug>")]
pub fn create_bug(
    bug: Json<NewBug>,
    conn: DbConn,
) -> Result<status::Created<Json<BugDto>>, Status> {
    bug_service::create_bug(bug.into_inner(), conn)
}

#[put("/<id>", data = "<bug>")]
pub fn update_bug(id: i32, bug: Json<BugDto>, connection: DbConn) -> Result<Json<BugDto>, Status> {
    bug_service::update_bug(id, bug.into_inner(), connection).map(|bug| Json(bug))
}

#[delete("/<id>")]
pub fn delete_bug(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    bug_service::delete_bug(id, connection)
}
