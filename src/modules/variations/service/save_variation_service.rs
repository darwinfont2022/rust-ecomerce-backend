use diesel::{Connection, QueryDsl, ExpressionMethods};
use diesel::prelude::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use crate::modules::variations::model::domain::new_variation_price::NewVariationPrice;
use crate::modules::variations::model::domain::variation::{Variation, VariationPrice};
use crate::modules::variations::model::domain::variation_new::VariationNew;
use crate::modules::variations::model::dto::variation_dto::{VariationDto, VariationDtoRes};
use crate::modules::variations::model::dto::variation_price_dto::VariationPriceDtoRes;
use crate::schema::variations::dsl::variations;

impl Variation {
    pub fn save_variation(conn: &mut PgConnection, variation_new: VariationNew) -> Result<Variation, DieselError> {
        use crate::schema::variations::dsl::*;

        diesel::insert_into(variations)
        .values(variation_new)
            .returning(Variation::as_returning())
            .get_result(conn)
    }

    pub fn save_full_variation(conn: &mut PgConnection, variation_dto: VariationDto) -> Result<VariationDtoRes, DieselError>{
        conn.transaction::<_, DieselError, _>(|conn| {
            let variation_new: VariationNew = variation_dto.clone().into();
            let variation_saved = Self::save_variation(conn, variation_new)?;

            let mut price_new: NewVariationPrice = variation_dto.price.into();
            price_new.variation_id = variation_saved.variation_id;

            let price_saved = VariationPrice::save_variation_price(conn, price_new)?;

            Ok(VariationDtoRes {
                variation_id: variation_saved.variation_id,
                product_id: variation_saved.product_id,
                available_quantity: variation_saved.available_quantity,
                sold_quantity: variation_saved.sold_quantity,
                catalog_product_id: variation_saved.catalog_product_id,
                price: Some(price_saved.into())
            })
        })
    }

    pub fn save_all_variations(conn: &mut PgConnection, all_variations: Vec<VariationNew>) -> Result<Vec<Variation>, DieselError> {
        use crate::schema::variations::dsl::*;

        diesel::insert_into(variations)
        .values(&all_variations)
            .returning(Variation::as_returning())
            .get_results(conn)
    }
}