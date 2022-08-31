use axum::{routing::post, Router};

pub fn get_routes() -> Router {
    Router::new().route("/price-items", post(create_price_item_handler))
}

async fn create_price_item_handler() -> &'static str {
    "CREATED"
}
