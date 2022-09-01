use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Router,
};
use bson::doc;
use futures::TryStreamExt;
use mongodb::options::FindOptions;

use crate::{
    infra::databases::mongo::get_collection,
    lib::{
        custom_response::{CustomResponse, CustomResponseBuilder, ResultCode},
        query_pagination::QueryPagination,
        validated_dto::ValidatedDto,
    },
};

use super::{
    dto::{CreatePriceItemDto, PriceItemDto, PriceItemsFilter},
    models::PriceItemModel,
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/price-items", post(create_price_item_handler))
        .route("/price-items/:id", get(get_price_item))
        .route("/price-items", get(get_price_items))
}

async fn get_price_item(Path(id): Path<String>) -> CustomResponse<PriceItemDto> {
    let collection = get_collection::<PriceItemModel>("price_items");

    if let Some(price_item) = collection
        .find_one(doc! { "_id": &id}, None)
        .await
        .expect("Get price item failed!")
    {
        return CustomResponseBuilder::new().data(price_item.into()).build();
    } else {
        return CustomResponseBuilder::new()
            .result_code(ResultCode::Err)
            .message(&format!("Price item with id {} not found!", id))
            .build();
    }
}

async fn get_price_items(
    pagination: Option<Query<QueryPagination>>,
    Query(filters): Query<PriceItemsFilter>,
) -> CustomResponse<Vec<PriceItemDto>> {
    let pagination = pagination.unwrap_or_default();

    let collection = get_collection::<PriceItemModel>("price_items");

    let mut filter = doc! {};

    if let Some(name) = filters.name {
        filter.insert("name", name);
    }

    if let Some(item_number) = filters.item_number {
        filter.insert("item_number", item_number);
    }

    let find_options = FindOptions::builder()
        .limit(pagination.limit())
        .skip(pagination.skip())
        .build();

    let mut cursor = collection.find(filter, find_options).await.unwrap();

    let mut price_items: Vec<PriceItemDto> = vec![];

    while let Some(price_item) = cursor.try_next().await.unwrap() {
        price_items.push(price_item.into());
    }

    CustomResponseBuilder::new().data(price_items).build()
}

async fn create_price_item_handler(
    ValidatedDto(dto): ValidatedDto<CreatePriceItemDto>,
) -> CustomResponse<PriceItemDto> {
    let collection = get_collection::<PriceItemModel>("price_items");

    if let Some(_) = collection
        .find_one(
            doc! {
                "$or": [
                    {"item_number": &dto.item_number},
                    {"name": &dto.name}
                ]
            },
            None,
        )
        .await
        .expect("Get price item failed!")
    {
        return CustomResponseBuilder::new()
            .result_code(ResultCode::Err)
            .message("Price item with this name or item_number already exists!")
            .build();
    }

    let mut model: PriceItemModel = dto.into();

    let res = collection.insert_one(&model, None).await;

    if let Ok(insert_one_result) = res {
        model._id = insert_one_result.inserted_id.as_object_id();
        return CustomResponseBuilder::new()
            .data(model.into())
            .message("Элемент прайс листа успешно создан!")
            .build();
    } else {
        return CustomResponseBuilder::new()
            .result_code(ResultCode::Err)
            .message("Что-то пошло не так!")
            .build();
    }
}
