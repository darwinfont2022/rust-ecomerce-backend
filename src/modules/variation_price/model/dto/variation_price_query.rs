use serde::Deserialize;

#[derive(Deserialize)]
pub struct VariationPriceQuery {
    pub variation_id: i32,
    pub detail: Option<bool>,
}