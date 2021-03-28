use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BugDto {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub resolved: bool,
}
