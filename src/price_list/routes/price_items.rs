use axum::{
    extract::{Path, Query},
    routing::post,
    Json, Router,
};

use crate::{
    lib::{
        custom_response::{CustomResponse, IntoCustomResponse},
        query_pagination::QueryPagination,
    },
    price_list::dto::{CreatePriceItemDto, PriceItemDto, PriceItemsFilters},
};

pub fn get_routes() -> Router {
    Router::new().route("/price-items", post(create_price_item_handler))
}

async fn create_price_item_handler(
    Json(dto): Json<CreatePriceItemDto>,
) -> CustomResponse<PriceItemDto> {
    PriceItemDto {
        id: String::from("test"),
        name: String::from("Перчатки"),
        item_number: 1,
        materials_cost: 10.0,
    }
    .into_custom_response()
}

async fn get_price_item_handler(Path(id): Path<String>) -> CustomResponse<PriceItemDto> {
    PriceItemDto {
        id: String::from("test"),
        name: String::from("Перчатки"),
        item_number: 1,
        materials_cost: 10.0,
    }
    .into_custom_response()
}

async fn get_price_items_handler(
    pagination: Option<Query<QueryPagination>>,
    Query(filters): Query<PriceItemsFilters>,
) -> &'static str {
    let pagination = pagination.unwrap_or_default();
    "OK"
}
