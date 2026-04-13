use mongodb::{Client, Database};
use std::env;

pub async fn connect_db() -> Database {
    let uri = env::var("MONGODB_URI")
        .expect("MONGODB_URI not set");

    let db_name = env::var("DATABASE_NAME")
        .expect("DATABASE_NAME not set");

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect");

    client.database(&db_name)
}
