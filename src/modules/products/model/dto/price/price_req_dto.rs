use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceReqDTO {
    pub price: BigDecimal,
    #[serde(rename = "basePrice")]
    pub base_price: Option<BigDecimal>,
    #[serde(rename = "originalPrice")]
    pub original_price: Option<BigDecimal>,
    #[serde(rename = "currencyId")]
    pub currency_id: String,
}