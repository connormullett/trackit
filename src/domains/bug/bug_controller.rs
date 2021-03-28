use super::{bug_service, BugDto, NewBug};
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_all_bugs() -> Json<Vec<BugDto>> {
    Json(bug_service::get_all_bugs())
}

#[post("/", data = "<bug>")]
pub fn create_bug(bug: Json<NewBug>) -> Json<BugDto> {
    Json(bug_service::create_bug(bug.into_inner()))
}
