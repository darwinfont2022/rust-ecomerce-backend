use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::db::utils::now;
use crate::modules::variation_price::model::domain::variation_price::{NewVariationPrice, VariationPrice, VariationPriceUpdate};

#[derive(Deserialize, Clone,Debug)]
pub struct VariationPriceDto {
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
    pub price_type: Option<String>,
}

#[derive(Deserialize, Clone,Debug)]
pub struct VariationPriceUpdateReq {
    pub variation_id: Option<i32>,
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

impl Into<NewVariationPrice> for VariationPriceDto {
    fn into(self) -> NewVariationPrice {
        NewVariationPrice {
            variation_id: 0,
            price: self.price.clone(),
            base_price: match self.base_price {
                Some(b) => Some(b),
                None => self.price.clone(),
            },
            original_price: match self.original_price {
                Some(o) => Some(o),
                None => self.price,
            },
            currency_id: match self.currency_id {
                Some(c) => Some(c),
                None => Some("USD".to_string()),
            },
            price_type: match self.price_type {
                Some(p) => Some(p),
                None => Some("original".to_string()),
            },
            create_at: Some(now())
        }
    }
}

impl Into<VariationPriceUpdate> for VariationPriceUpdateReq {
    fn into(self) -> VariationPriceUpdate {
        VariationPriceUpdate {
            price_id: 0,
            variation_id: self.variation_id.unwrap_or(0),
            price: self.price.clone(),
            base_price: self.base_price,
            original_price: self.original_price,
            currency_id: self.currency_id,
            price_type: self.price_type,
            update_at: Some(now()),
        }
    }
}

impl From<VariationPrice> for VariationPriceDtoRes {
    fn from(variation_price: VariationPrice) -> Self {
        Self {
            price_id: variation_price.variation_id,
            price: variation_price.price,
            base_price: variation_price.base_price,
            original_price: variation_price.original_price,
            price_type: variation_price.price_type,
            currency_id: variation_price.currency_id,
            create_at: variation_price.create_at,
            update_at: variation_price.update_at,
        }
    }
}