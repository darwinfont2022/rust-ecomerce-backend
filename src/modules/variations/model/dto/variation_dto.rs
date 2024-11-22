use serde::{Deserialize, Serialize};
use crate::modules::attribute_combinations::model::domain::attribute_combination::AttributeCombination;
use crate::modules::variation_price::model::dto::variation_price_dto::{VariationPriceDto, VariationPriceDtoRes};
use crate::modules::attribute_combinations::model::dto::attribute_combination_dto::{AttributeCombinationReq, AttributeCombinationRes};
use crate::modules::variations::model::domain::variation::Variation;
use crate::modules::variation_price::model::domain::variation_price::VariationPrice;

#[derive(Deserialize, Clone ,Debug)]
pub struct VariationDto {
    pub product_id: i32,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub catalog_product_id: Option<String>,
    pub price: VariationPriceDto,
    pub attributes_variations: Option<Vec<AttributeCombinationReq>>,
}

#[derive(Serialize, Debug)]
pub struct VariationDtoRes {
    #[serde(flatten)]
    pub variation: Variation,

    pub price: VariationPrice,
    pub attributes_variations: Vec<AttributeCombination>,
}

impl VariationDtoRes {
    pub fn new() -> Self {
        Self {
            variation: Variation::new(),
            price: VariationPrice::new(),
            attributes_variations: Vec::new(),
        }
    }
}
