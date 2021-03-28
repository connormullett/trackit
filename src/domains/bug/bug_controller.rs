use super::BugDto;
use rocket_contrib::json::Json;

#[get("/")]
pub fn find_all_bugs() -> Json<BugDto> {
    Json(BugDto {
        title: "hello".to_string(),
        text: "bug".to_string(),
    })
}
