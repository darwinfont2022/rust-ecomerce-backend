use crate::modules::variations::model::domain::variation::VariationPrice;
use crate::modules::variations::model::dto::variation_price_dto::VariationPriceDto;
use diesel::result::Error as DieselError;
use diesel::prelude::{PgConnection, RunQueryDsl, SelectableHelper};
use crate::modules::variations::model::domain::new_variation_price::NewVariationPrice;
use crate::schema::variation_price::dsl::variation_price;

impl VariationPrice {
    pub fn save_variation_price(conn: &mut PgConnection, price_new: NewVariationPrice) -> Result<Self, DieselError> {

        diesel::insert_into(variation_price)
            .values(&price_new)
            .returning(VariationPrice::as_returning())
            .get_result(conn)
    }
}