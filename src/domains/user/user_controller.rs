use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;

use crate::connection::DbConn;

use super::{user_service, NewUser};

#[post("/", data = "<user>")]
pub fn register(
    user: Json<NewUser>,
    connection: DbConn,
) -> Result<status::Created<Json<String>>, Status> {
    user_service::create_user(user.into_inner(), connection)
}
