use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuantityResDto {
    #[serde(rename(serialize = "initialQuantity"))]
    pub initial_quantity: i32,
    #[serde(rename(serialize = "availableQuantity"))]
    pub available_quantity: Option<i32>,
    #[serde(rename(serialize = "soldQuantity"))]
    pub sold_quantity: Option<i32>,
}