use bigdecimal::BigDecimal;
use diesel::{Identifiable, Queryable, Selectable};
use serde::Serialize;
use crate::schema::variations::dsl::*;

#[derive(Queryable, Selectable, Identifiable, Serialize, Debug)]
#[diesel(table_name = crate::schema::variations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(variation_id))]
pub struct Variation {
    pub variation_id: i32,
    pub product_id: Option<i32>,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub catalog_product_id: Option<String>,
    // pub create_at: Option<chrono::NaiveDateTime>,
    // pub update_at: Option<chrono::NaiveDateTime>,
}
#[derive(Queryable, Selectable, Identifiable, Serialize, Debug)]
#[diesel(table_name = crate::schema::variation_price)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(price_id))]
pub struct VariationPrice {
    pub price_id: i32,
    pub variation_id: i32,
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
    pub price_type: Option<String>,
    pub create_at: Option<chrono::NaiveDateTime>,
    pub update_at: Option<chrono::NaiveDateTime>,
}