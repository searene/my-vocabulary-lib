mod initializer {
    use diesel::{SqliteConnection, Connection};
    use std::env;

    fn establish_db_connection() -> SqliteConnection {
        dotenv().ok();
        let db_url= env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");
        SqliteConnection::establish(&db_url)
            .expect(&format!("Error connecting to {}", &db_url))
    }
}