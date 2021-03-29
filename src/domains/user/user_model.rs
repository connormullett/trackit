use crate::schema::app_user;
use serde::Deserialize;

#[derive(Queryable, AsChangeset)]
#[table_name = "app_user"]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "app_user"]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}
