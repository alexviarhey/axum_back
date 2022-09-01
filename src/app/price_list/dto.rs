use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePriceItemDto {
    #[validate(range(min = 1, message = "Item number must be grater than 0"))]
    pub item_number: u32,

    #[validate(length(min = 1, message = "Name length must be grater than 1"))]
    pub name: String,

    #[validate(range(min = 0.01, message = "Material cost must be grater than 0.01"))]
    pub material_cost: f64,
}

#[derive(Debug, Serialize)]
pub struct PriceItemDto {
    pub id: String,
    pub item_number: u32,
    pub name: String,
    pub material_cost: f64,
}
