use mongodb::{Client, Collection, Database};

static mut CONNECTION: Option<Database> = None;

pub async fn connect(uri: &str, db_name: &str) {
    let connetion = Client::with_uri_str(uri)
        .await
        .expect("Unable to connect to mongodb")
        .database(db_name);

    println!("Connect to mongodb!");

    unsafe { CONNECTION = Some(connetion) }
}

pub fn get_collection<T>(name: &str) -> Collection<T> {
    let connection = get_connection();
    connection.collection::<T>(name)
}

fn get_connection() -> &'static Database {
    unsafe {
        CONNECTION
            .as_ref()
            .expect("Mongo connection not initialized!")
    }
}
