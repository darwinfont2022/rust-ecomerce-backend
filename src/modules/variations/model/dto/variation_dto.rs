use serde::{Deserialize, Serialize};
use crate::modules::variations::model::dto::variation_price_dto::{VariationPriceDto, VariationPriceDtoRes};

#[derive(Deserialize, Clone ,Debug)]
pub struct VariationDto {
    pub product_id: i32,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub catalog_product_id: Option<String>,
    pub price: VariationPriceDto
}

#[derive(Serialize, Debug)]
pub struct VariationDtoRes {
    pub variation_id: i32,
    pub product_id: Option<i32>,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub catalog_product_id: Option<String>,
    pub price: Option<VariationPriceDtoRes>
}

impl VariationDtoRes {
    pub fn new() -> Self {
        Self {
            variation_id: 0,
            product_id: None,
            available_quantity: None,
            sold_quantity: None,
            catalog_product_id: None,
            price: None,
        }
    }
}
