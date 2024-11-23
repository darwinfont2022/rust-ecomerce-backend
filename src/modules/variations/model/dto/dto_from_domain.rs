use crate::modules::variation_price::model::domain::variation_price::VariationPrice;
use crate::modules::variations::model::domain::variation::Variation;
use crate::modules::variations::model::dto::variation_dto::VariationDtoRes;

impl From<Variation> for VariationDtoRes {
    fn from(variation: Variation) -> Self {
        VariationDtoRes {
            variation,
            price: VariationPrice::new(),
            attributes_variations: Vec::new(),
        }
    }
}