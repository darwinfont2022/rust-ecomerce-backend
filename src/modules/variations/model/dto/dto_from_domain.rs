use crate::modules::variations::model::domain::new_variation_price::NewVariationPrice;
use crate::modules::variations::model::domain::variation::{Variation, VariationPrice};
use crate::modules::variations::model::dto::variation_dto::VariationDtoRes;
use crate::modules::variations::model::dto::variation_price_dto::VariationPriceDtoRes;

impl From<Variation> for VariationDtoRes {
    fn from(variation: Variation) -> Self {
        VariationDtoRes {
            variation_id: variation.variation_id,
            product_id: variation.product_id,
            available_quantity: variation.available_quantity,
            sold_quantity: variation.sold_quantity,
            catalog_product_id: variation.catalog_product_id,
            price: None,
        }
    }
}

impl From<VariationPrice> for VariationPriceDtoRes {
    fn from(variation_price: VariationPrice) -> Self {
        Self {
            price_id: variation_price.variation_id,
            price: variation_price.price,
            base_price: variation_price.base_price,
            original_price: variation_price.original_price,
            price_type: variation_price.price_type,
            currency_id: variation_price.currency_id,
            create_at: variation_price.create_at,
            update_at: variation_price.update_at,
        }
    }
}