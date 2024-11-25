use diesel::{Connection, PgConnection, QueryDsl, QueryResult, RunQueryDsl, GroupedBy, ExpressionMethods, SelectableHelper};
use diesel::result::Error as DieselError;
use crate::modules::variations::model::domain::variation::Variation;
use crate::modules::attribute_combinations::model::domain::attribute_combination::AttributeCombination;
use crate::modules::variations::model::dto::variation_dto::VariationDtoRes;
use crate::modules::variation_price::model::domain::variation_price::VariationPrice;

impl Variation {
    pub fn find_variation_detail(
        conn: &mut PgConnection, id: i32
    ) -> Result<VariationDtoRes, DieselError> {
        conn.transaction(|conn| {
            let variation = Self::find_variation_by_id(conn, id)?;
            let prices = VariationPrice::find_all_by_variation(conn, id)?;
            let attributes_variations = AttributeCombination::find_combination_by_variation_id(conn, id)?;
            Ok(VariationDtoRes{
                variation,
                price: prices,
                attributes_variations,
            })
        })
    }
    pub fn find_variation_by_id(conn: &mut PgConnection, id: i32) -> Result<Variation, DieselError> {
        use crate::schema::variations::dsl::*;

        variations
            .select(Variation::as_select())
            .find(id)
            .get_result(conn)
    }
    pub fn find_by_product_id(conn: &mut PgConnection, id: i32) -> Result<Vec<Variation>, DieselError> {
        use crate::schema::{variations::dsl::*};
        variations.filter(product_id.eq(id)).select(Variation::as_select()).load::<Variation>(conn)
    }
    pub fn find_all_by_product_id(conn: &mut PgConnection, id: i32) -> Result<Vec<VariationDtoRes>, DieselError> {
        conn.transaction(|conn| {
            let all_variations = Self::find_by_product_id(conn, id)?;
            let combinations = AttributeCombination::find_all_by_parents(conn, &all_variations)?;
            let prices = VariationPrice::find_all_by_parents(conn, &all_variations)?;
            let dto_res = Self::map_combinations(combinations, all_variations.clone());
            let resp = Self::map_prices(all_variations, prices, dto_res);

            Ok(resp)
        })
    }

    fn map_combinations(combinations: Vec<AttributeCombination>, all_variations: Vec<Variation>) -> Vec<VariationDtoRes> {
        combinations.grouped_by(&all_variations)
            .into_iter()
            .zip(all_variations)
            .map(|(attributes_variations, variation)| VariationDtoRes {
                variation,
                price: Vec::new(),
                attributes_variations,
            })
            .collect::<Vec<VariationDtoRes>>()
    }

    pub fn map_prices(all_variations: Vec<Variation>, prices: Vec<VariationPrice>, dto_res: Vec<VariationDtoRes>) -> Vec<VariationDtoRes> {
        prices.grouped_by(&all_variations)
            .into_iter()
            .zip(dto_res)
            .map(|(k, mut v)| {
                v.price = k.clone();
                v
            })
            .collect::<Vec<VariationDtoRes>>()
    }
}