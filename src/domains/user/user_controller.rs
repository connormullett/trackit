use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;

use crate::connection::DbConn;

use super::{user_service, AuthResponse, UserDto};

#[post("/", data = "<user>")]
pub fn register(
    user: Json<UserDto>,
    connection: DbConn,
) -> Result<status::Created<Json<AuthResponse>>, Status> {
    user_service::create_user(user.into_inner(), connection)
}
