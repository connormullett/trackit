use rocket::Outcome;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};

pub extern crate crypto;
pub extern crate jwt;
pub extern crate rustc_serialize;

use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};

use crate::{
    connection::DbConn,
    domains::{user_repository, UserDto},
};

pub struct ApiKey(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        match validate_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(()),
        }
    }
}

pub fn validate_token(key: &str) -> Result<String, String> {
    let secret_key = dotenv::var("JWT_SIGNING_KEY").expect("JWT_SIGNING_KEY is required");
    let token =
        Token::<Header, Registered>::parse(key).map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(secret_key.as_bytes(), Sha256::new()) {
        token.claims.sub.ok_or("Claims are invalid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

pub fn issue_auth_token(credentials: UserDto, conn: DbConn) -> Result<String, Status> {
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
