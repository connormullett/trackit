use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

use crate::schema::app_user;

use super::{Credentials, UserInsert, UserModel};

pub fn create_user(user: UserInsert, connection: &PgConnection) -> QueryResult<UserModel> {
    diesel::insert_into(app_user::table)
        .values(&user)
        .get_result(connection)
}

pub fn authenticate_user(credentials: Credentials, connection: &PgConnection) -> Option<UserModel> {
    println!("authenticate_user password {}", credentials.password);
    app_user::table
        .filter(app_user::username.eq(credentials.username))
        .filter(app_user::password_hash.eq(credentials.password))
        .order(app_user::id)
        .first(connection)
        .map(|user| Some(user))
        .unwrap_or(None)
}
