use crate::schema::bug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BugDto {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub resolved: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "bug"]
pub struct NewBug<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
