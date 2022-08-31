use mongodb::{Client, Database};

pub async fn connect(uri: &str, db_name: &str) -> Database {
    Client::with_uri_str(uri)
        .await
        .expect("Unable to connect to mongodb")
        .database(db_name)
}
