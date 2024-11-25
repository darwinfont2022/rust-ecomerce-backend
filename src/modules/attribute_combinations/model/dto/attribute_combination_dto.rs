use serde::{Deserialize, Serialize};
use crate::modules::attribute_combinations::model::domain::attribute_combination::{AttributeCombination, AttributeCombinationUpdate};

#[derive(Deserialize, Debug, Clone)]
pub struct AttributeCombinationReq {
    pub combination_external_id: Option<String>,
    pub combination_name: Option<String>,
    pub combination_value_id: Option<String>,
    pub combination_value_name: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct AttributeCombinationRes {
    pub combination_id: i32,
    pub variation_id: i32,
    pub combination_external_id: Option<String>,
    pub combination_name: Option<String>,
    pub combination_value_id: Option<String>,
    pub combination_value_name: Option<String>,
}

impl Into<AttributeCombinationUpdate> for AttributeCombinationReq {
    fn into(self) -> AttributeCombinationUpdate {
        AttributeCombinationUpdate {
            combination_id: 0,
            variation_id: None,
            combination_external_id: self.combination_external_id,
            combination_name: self.combination_name,
            combination_value_id: self.combination_value_id,
            combination_value_name: self.combination_value_name,
        }
    }
}

impl From<AttributeCombination> for AttributeCombinationRes {
    fn from(attribute: AttributeCombination) -> Self {
        AttributeCombinationRes {
            combination_id: attribute.combination_id,
            variation_id: attribute.variation_id,
            combination_external_id: attribute.combination_external_id,
            combination_name: attribute.combination_name,
            combination_value_id: attribute.combination_value_id,
            combination_value_name: attribute.combination_value_name,
        }
    }
}