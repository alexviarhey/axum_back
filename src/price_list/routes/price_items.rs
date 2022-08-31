use axum::{extract::Query, routing::post, Router};

use crate::{
    lib::{query_pagination::QueryPagination, validated_dto::ValidatedDto},
    price_list::dto::{CreatePriceItemDto, PriceItemsFilters},
};

pub fn routes() -> Router {
    Router::new().route(
        "/price-items",
        post(create_price_item_handler).get(get_price_item_handler),
    )
}

async fn create_price_item_handler(
    ValidatedDto(dto): ValidatedDto<CreatePriceItemDto>,
) -> &'static str {
    println!("{:?}", dto);

    "CREATED"
}

async fn get_price_item_handler() {}

async fn get_price_items_handler(
    pagination: Option<Query<QueryPagination>>,
    Query(filters): Query<PriceItemsFilters>,
) -> &'static str {
    let pagination = pagination.unwrap_or_default();
    "OK"
}
