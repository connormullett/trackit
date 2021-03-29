use crypto::sha2::Sha256;
use diesel::result::Error;
use jwt::{Header, Registered, Token};
use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;
use serde::Deserialize;

use crate::connection::DbConn;

use super::{user_repository, NewUser, UserModel};

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn create_user(user: NewUser, conn: DbConn) -> Result<status::Created<Json<String>>, Status> {
    let user = user_repository::create_user(user, &conn);

    match user {
        Ok(user) => user_created(user, conn),
        Err(error) => Err(error_status(error)),
    }
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn issue_auth_token(credentials: Credentials, conn: DbConn) -> Result<String, Status> {
    let header: Header = Default::default();

    match user_repository::authenticate_user(credentials, &conn) {
        None => Err(Status::NotFound),
        Some(user) => {
            let claims = Registered {
                sub: Some(user.username.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token
                .signed(
                    dotenv::var("JWT_SIGNING_KEY")
                        .expect("JWT_SIGNING_KEY is required")
                        .as_bytes(),
                    Sha256::new(),
                )
                .map(|message| message)
                .map_err(|_| Status::InternalServerError)
        }
    }
}

fn user_created(user: UserModel, conn: DbConn) -> Result<status::Created<Json<String>>, Status> {
    match issue_auth_token(
        Credentials {
            username: user.username,
            password: user.password_hash,
        },
        conn,
    ) {
        Ok(token) => Ok(status::Created(
            format!("/users/{}", user.id),
            Some(Json(token)),
        )),
        Err(_) => Err(Status::NotFound),
    }
}
