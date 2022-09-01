use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::dto::{CreatePriceItemDto, PriceItemDto};

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceItemModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    item_number: u32,
    name: String,
    material_cost: f64,
}

impl From<CreatePriceItemDto> for PriceItemModel {
    fn from(dto: CreatePriceItemDto) -> Self {
        Self {
            _id: None,
            item_number: dto.item_number,
            name: dto.name,
            material_cost: dto.material_cost,
        }
    }
}

impl Into<PriceItemDto> for PriceItemModel {
    fn into(self) -> PriceItemDto {
        PriceItemDto {
            id: self
                ._id
                .expect("Expected _id in price item model")
                .to_string(),
            item_number: self.item_number,
            name: self.name,
            material_cost: self.material_cost,
        }
    }
}
