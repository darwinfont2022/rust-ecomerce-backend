use diesel::Insertable;
use crate::modules::attribute_combinations::model::dto::attribute_combination_dto::AttributeCombinationReq;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::attribute_combinations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(combination_id))]
pub struct AttributeCombinationNew {
    pub variation_id : i32,
    pub combination_external_id : Option<String>,
    pub combination_name : Option<String>,
    pub combination_value_id : Option<String>,
    pub combination_value_name : Option<String>,
}

impl From<AttributeCombinationReq> for AttributeCombinationNew {
    fn from(value: AttributeCombinationReq) -> Self {
        AttributeCombinationNew {
            variation_id: 0,
            combination_external_id: value.combination_external_id,
            combination_name: value.combination_name,
            combination_value_id: value.combination_value_id,
            combination_value_name: value.combination_value_name,
        }
    }
}