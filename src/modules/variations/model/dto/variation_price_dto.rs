use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::schema::product_price::create_at;

#[derive(Deserialize, Clone,Debug)]
pub struct VariationPriceDto {
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
    pub price_type: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct VariationPriceDtoRes {
    pub price_id: i32,
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
    pub price_type: Option<String>,
    pub create_at: Option<chrono::NaiveDateTime>,
    pub update_at: Option<chrono::NaiveDateTime>
}