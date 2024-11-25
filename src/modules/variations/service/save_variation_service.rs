use diesel::Connection;
use diesel::prelude::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel::result::Error as DieselError;
use crate::modules::attribute_combinations::model::domain::attribute_combination::AttributeCombination;
use crate::modules::variations::model::domain::variation::Variation;
use crate::modules::variation_price::model::domain::variation_price::{VariationPrice, NewVariationPrice};
use crate::modules::variations::model::domain::variation_new::VariationNew;
use crate::modules::variations::model::dto::variation_dto::{VariationDto, VariationDtoRes};

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
            let combinations = AttributeCombination::save_all_combination(conn, variation_saved.variation_id,variation_dto.attributes_variations.unwrap_or_else(Vec::new))?;
            let price_saved = VariationPrice::save(conn, variation_dto.price.into(), variation_saved.variation_id)?;

            Ok(VariationDtoRes {
                variation: variation_saved,
                price: Vec::new(),//price_saved.into(),
                attributes_variations: combinations,
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