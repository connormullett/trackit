use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;

use serde_json::Value;

use serde_json::json;

use crate::{connection::DbConn, domains::ApiKey};

use super::{user_service, AuthResponse, UserDto};

#[post("/", data = "<user>")]
pub fn register(
    user: Json<UserDto>,
    connection: DbConn,
) -> Result<status::Created<Json<AuthResponse>>, Status> {
    user_service::create_user(user.into_inner(), connection)
}

#[get("/me")]
pub fn get_me(_api_key: ApiKey) -> Json<Value> {
    Json(json!({
        "success": "true"
    }))
}
