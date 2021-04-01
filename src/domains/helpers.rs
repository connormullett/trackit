use diesel::result::{DatabaseErrorKind, Error};
use rocket::http::Status;

pub fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        Error::DatabaseError(kind, _) => match kind {
            DatabaseErrorKind::UniqueViolation => Status::Conflict,
            _ => Status::InternalServerError,
        },
        _ => Status::InternalServerError,
    }
}
