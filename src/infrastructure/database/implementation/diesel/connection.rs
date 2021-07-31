use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;
use dotenv::dotenv;
use diesel::SqliteConnection;
use diesel::Connection;

lazy_static! {
    pub static ref CONNECTION: Mutex<SqliteConnection> =
            Mutex::new(establish_connection());
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
