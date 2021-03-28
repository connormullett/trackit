use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BugDto {
    pub title: String,
    pub text: String,
}
