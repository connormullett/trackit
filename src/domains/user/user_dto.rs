use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserDto {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}
