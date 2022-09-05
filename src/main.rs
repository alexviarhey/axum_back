mod app;
mod infra;
mod lib;

use app::price_list;
use axum::Router;
use infra::{
    config::{Config, ServiceConfig},
    databases::mongo,
};
use mongodb::Database;

#[tokio::main]
async fn main() {
    let state = run_infrastructure().await;
    let addr = format!("{}:{}", state.service.address, state.service.port);

    println!("Server started at {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(create_router().into_make_service())
        .await
        .unwrap();
}

async fn run_infrastructure() -> State {
    let config = Config::new();

    let db = mongo::connect(&config.mongo.mongo_uri, &config.mongo.db_name).await;

    State {
        db,
        service: config.service,
    }
}

fn create_router() -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new().merge(price_list::routes::get_routes()),
    )
}

#[derive(Debug)]
struct State {
    db: &'static Database,
    service: ServiceConfig,
}
