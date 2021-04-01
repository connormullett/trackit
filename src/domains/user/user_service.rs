use helpers::error_status;
use pwhash::bcrypt;
use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::domains::auth::auth_service;
use crate::domains::helpers;

use super::{user_repository, AuthResponse, UserDto, UserInsert, UserModel};

pub fn create_user(
    new_user: UserDto,
    conn: DbConn,
) -> Result<status::Created<Json<AuthResponse>>, Status> {
    let user = UserInsert {
        username: new_user.username,
        password_hash: bcrypt::hash(new_user.password).unwrap(),
    };

    let user = user_repository::create_user(user, &conn);

    match user {
        Ok(user) => user_created(user, conn),
        Err(error) => Err(error_status(error)),
    }
}

fn user_created(
    user: UserModel,
    conn: DbConn,
) -> Result<status::Created<Json<AuthResponse>>, Status> {
    match auth_service::issue_auth_token(
        UserDto {
            username: user.username,
            password: user.password_hash,
        },
        conn,
    ) {
        Ok(token) => Ok(status::Created(
            format!("/users/{}", user.id),
            Some(Json(AuthResponse { token })),
        )),
        Err(_) => Err(Status::NotFound),
    }
}
