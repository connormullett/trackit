use diesel::{pg::PgConnection, Connection};
use dotenv;

pub fn estabilish_connection() -> PgConnection {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
