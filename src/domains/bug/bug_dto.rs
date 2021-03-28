use crate::schema::bug;
use serde::{Deserialize, Serialize};

use super::BugModel;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "bug"]
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

impl BugDto {
    pub fn from_model(model: BugModel) -> BugDto {
        BugDto {
            id: model.id,
            title: model.title,
            body: model.body,
            resolved: model.resolved,
        }
    }
}
