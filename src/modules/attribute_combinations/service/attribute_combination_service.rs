use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, ExpressionMethods};
use diesel::result::Error as DieselError;
use crate::modules::attribute_combinations::model::domain::attribute_combination::AttributeCombination;
use crate::modules::attribute_combinations::model::domain::attribute_combination_news::AttributeCombinationNew;
use crate::modules::attribute_combinations::model::dto::attribute_combination_dto::AttributeCombinationReq;

impl AttributeCombination {
    pub fn save_all_combination(conn: &mut PgConnection, variation: i32 ,combinations_dto: Vec<AttributeCombinationReq>) -> Result<Vec<AttributeCombination>, DieselError> {
        use crate::schema::attribute_combinations::dsl::*;

        let combinations: Vec<AttributeCombinationNew> = combinations_dto.into_iter().map(|c| {
            let mut new_combination = AttributeCombinationNew::from(c);
            new_combination.variation_id = variation;
            new_combination
        }).collect();

        diesel::insert_into(attribute_combinations)
            .values(&combinations)
            .returning(AttributeCombination::as_returning())
            .get_results(conn)
    }

    pub fn find_combination_by_variation_id(conn: &mut PgConnection, id: i32) -> Result<Vec<AttributeCombination>, DieselError> {
        use crate::schema::attribute_combinations::dsl::*;

        attribute_combinations
            .select(AttributeCombination::as_returning())
            .filter(variation_id.eq(id))
            .get_results::<AttributeCombination>(conn)
    }

    // pub fn group_by_attribute_combinations(conn: &mut PgConnection) -> Vec<AttributeCombination> {
    //
    // }
}