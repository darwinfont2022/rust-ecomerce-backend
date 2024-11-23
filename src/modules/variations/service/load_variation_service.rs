use diesel::{Connection, PgConnection, QueryDsl, QueryResult, RunQueryDsl, BelongingToDsl};
use diesel::result::Error as DieselError;
use diesel::SelectableHelper;
use diesel::ExpressionMethods;
use crate::modules::variations::model::domain::variation::Variation;
use crate::modules::attribute_combinations::model::domain::attribute_combination::AttributeCombination;
use crate::modules::variations::model::dto::variation_dto::VariationDtoRes;
use diesel::GroupedBy;
use crate::modules::variation_price::model::domain::variation_price::VariationPrice;

impl Variation {
    pub fn find_variation_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Vec<Variation>> {
        use crate::schema::variations::dsl::*;

        variations.select(Variation::as_select()).find(id).load::<Variation>(conn)
    }

    pub fn find_all_by_product_id(conn: &mut PgConnection, id: i32) -> Result<Vec<VariationDtoRes>, DieselError> {
        use crate::schema::{variations::dsl::*};

        conn.transaction(|conn| {
            let all_variations = variations.filter(product_id.eq(id)).select(Variation::as_select()).load::<Variation>(conn)?;

            let combinations = AttributeCombination::belonging_to(&all_variations)
                .select(AttributeCombination::as_select())
                .load(conn)?;

            Ok(Self::full_combinations(combinations, all_variations))
        })
    }

    pub fn full_combinations(combinations: Vec<AttributeCombination>, all_variations: Vec<Variation>) -> Vec<VariationDtoRes> {
        combinations.grouped_by(&all_variations)
            .into_iter()
            .zip(all_variations)
            .map(|(attributes_variations, variation)| VariationDtoRes {
                variation,
                price: VariationPrice::new(),
                attributes_variations,
            })
            .collect::<Vec<VariationDtoRes>>()
    }
}