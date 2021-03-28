use super::BugDto;

pub fn find_all_bugs() -> BugDto {
    BugDto {
        title: "hello".to_string(),
        text: "bug".to_string(),
    }
}
