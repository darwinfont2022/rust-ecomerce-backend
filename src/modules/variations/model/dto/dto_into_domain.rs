use crate::modules::variations::model::domain::variation_new::VariationNew;
use crate::modules::variations::model::dto::variation_dto::VariationDto;

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
