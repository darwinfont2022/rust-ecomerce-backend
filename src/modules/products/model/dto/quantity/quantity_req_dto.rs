use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct QuantityReqDto {
    pub initial_quantity: i32,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
}