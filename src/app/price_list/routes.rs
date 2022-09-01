use axum::{routing::post, Router};
use bson::doc;

use crate::{
    infra::databases::mongo::get_collection,
    lib::{custom_response::CustomResponse, validated_dto::ValidatedDto},
};

use super::{
    dto::{CreatePriceItemDto, PriceItemDto},
    models::PriceItemModel,
};

pub fn get_routes() -> Router {
    Router::new().route("/price-items", post(create_price_item_handler))
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
        return CustomResponse::with_error_message(
            "Price item with this name or item_number already exists!",
        );
    }

    let mut model: PriceItemModel = dto.into();

    let res = collection.insert_one(&model, None).await;

    if let Ok(insert_one_result) = res {
        model._id = insert_one_result.inserted_id.as_object_id();
        return CustomResponse::success(model.into(), "Элемент прайс листа успешно создан!");
    } else {
        CustomResponse::with_error_message("Something went wrong!")
    }
}
