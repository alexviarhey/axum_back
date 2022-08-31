mod config;
mod infra;
mod lib;
mod price_list;

use axum::Router;
use config::Config;
use infra::databases::mongo;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

async fn app() -> Router {
    let config = Config::new();

    let database = mongo::connect(&config.mongo.mongo_uri, &config.mongo.db_name).await;

    initialize_routes()
}

fn initialize_routes() -> Router {
    Router::new().merge(price_list::routes::price_items::routes())
}
