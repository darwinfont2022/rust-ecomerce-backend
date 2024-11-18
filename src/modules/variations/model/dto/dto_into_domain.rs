use crate::modules::variations::model::domain::new_variation_price::NewVariationPrice;
use crate::modules::variations::model::domain::variation_new::VariationNew;
use crate::modules::variations::model::dto::variation_dto::VariationDto;
use crate::modules::variations::model::dto::variation_price_dto::VariationPriceDto;

impl Into<VariationNew> for VariationDto {
    fn into(self) -> VariationNew {
        VariationNew {
            product_id: Some(self.product_id),
            available_quantity: self.available_quantity,
            sold_quantity: self.sold_quantity,
            catalog_product_id: self.catalog_product_id,
        }
    }
}

impl Into<NewVariationPrice> for VariationPriceDto {
        fn into(self) -> NewVariationPrice {
        let now = chrono::Utc::now().naive_utc();
        NewVariationPrice{
            variation_id: 0,
            price: self.price,
            base_price: self.base_price,
            original_price: self.original_price,
            currency_id: self.currency_id,
            price_type: self.price_type,
            create_at: Some(now),
        }
    }
}
