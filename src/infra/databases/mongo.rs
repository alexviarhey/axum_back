use mongodb::{options::ClientOptions, Client, Collection, Database};

static mut CONNECTION: Option<Database> = None;

pub async fn connect(uri: &str, db_name: &str) -> &'static Database {
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Mongo uri parsing error");

    let connection = Client::with_options(client_options)
        .expect("Cannot create mongo options")
        .database(db_name);

    unsafe { CONNECTION = Some(connection) };

    get_connection()
}

pub fn get_collection<T>(name: &str) -> Collection<T> {
    let connection = get_connection();
    connection.collection::<T>(name)
}

pub fn get_connection() -> &'static Database {
    unsafe {
        CONNECTION
            .as_ref()
            .expect("Mongo connection not initialized!")
    }
}
