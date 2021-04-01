use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{
    connection::DbConn,
    domains::{auth_service, AuthResponse, UserDto},
};

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<UserDto>, connection: DbConn) -> Result<Json<AuthResponse>, Status> {
    auth_service::issue_auth_token(credentials.into_inner(), connection)
        .map(|token| Ok(Json(AuthResponse { token })))?
}
