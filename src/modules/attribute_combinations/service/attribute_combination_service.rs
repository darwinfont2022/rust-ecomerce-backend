use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, ExpressionMethods, GroupedBy, BelongingToDsl};
use diesel::result::Error as DieselError;
use crate::modules::attribute_combinations::model::domain::attribute_combination::{AttributeCombination, AttributeCombinationUpdate};
use crate::modules::attribute_combinations::model::domain::attribute_combination_news::AttributeCombinationNew;
use crate::modules::attribute_combinations::model::dto::attribute_combination_dto::AttributeCombinationReq;
use crate::modules::variations::model::domain::variation::Variation;
use crate::schema::attribute_combinations::dsl::attribute_combinations;

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

    pub fn find_all_by_parents(conn: &mut PgConnection, vtns: &Vec<Variation>) -> Result<Vec<AttributeCombination>, DieselError> {
        AttributeCombination::belonging_to(vtns)
            .select(AttributeCombination::as_select())
            .load(conn)
    }

    pub fn update(conn: &mut PgConnection, id: i32 , combination: AttributeCombinationReq) -> Result<AttributeCombination, DieselError> {
        use crate::schema::attribute_combinations::dsl::*;
        let mut combination: AttributeCombinationUpdate = combination.into();
        combination.combination_id = id;

        diesel::update(attribute_combinations.filter(combination_id.eq(id)))
            .set(combination)
            .returning(AttributeCombination::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, target_id: i32) -> Result<usize, DieselError> {
        use crate::schema::attribute_combinations::dsl::*;
        diesel::delete(attribute_combinations.filter(combination_id.eq(target_id)))
        .execute(conn)
    }
}