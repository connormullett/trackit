use crate::schema::app_user;

#[derive(Queryable, AsChangeset)]
#[table_name = "app_user"]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "app_user"]
pub struct UserInsert {
    pub username: String,
    pub password_hash: String,
}
