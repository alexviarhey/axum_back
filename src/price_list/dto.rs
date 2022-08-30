use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePriceItemDto {
    #[validate(range(min = 1, message = "Item number is required!"))]
    item_number: u32,

    #[validate(length(min = 1, message = "Name is required!"))]
    name: String,

    #[validate(range(min = 0.1, message = "Material cost is required!"))]
    material_cost: f64,
}

#[derive(Debug, Deserialize)]
pub struct PriceItemsFilters {
    item_number: Option<u32>,
    name: Option<String>,
    material_cost: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct PriceItemDto {
    pub id: String,
    pub item_number: u32,
    pub name: String,
    pub materials_cost: f64,
}
