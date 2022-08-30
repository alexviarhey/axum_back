use axum::Router;
use mongodb::{Client, Database};

mod config;
mod lib;
mod price_list;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

async fn app() -> Router {
    let config = config::Config::new();

    connect_to_mongo_db(&config.mongo).await;

    println!("Connected to mongodb!");

    Router::new().merge(price_list::routes::price_items::get_routes())
}

async fn connect_to_mongo_db(config: &config::MongoConfig) -> Database {
    Client::with_uri_str(&config.mongo_uri)
        .await
        .expect("Unable to connect to mongodb")
        .database(&config.db_name)
}
