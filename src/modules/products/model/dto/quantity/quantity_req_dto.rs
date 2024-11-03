use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuantityReqDto {
    #[serde(rename = "initialQuantity")]
    pub initial_quantity: i32,
    #[serde(rename = "availableQuantity")]
    pub available_quantity: Option<i32>,
}