use super::{bug_service, BugDto};
use rocket_contrib::json::Json;

#[get("/")]
pub fn find_all_bugs() -> Json<BugDto> {
    let bugs = bug_service::find_all_bugs();
    Json(bugs)
}
