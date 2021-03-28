#[derive(Queryable)]
pub struct BugModel {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub resolved: bool,
}
