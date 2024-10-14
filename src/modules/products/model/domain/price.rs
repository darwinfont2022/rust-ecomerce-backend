use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::product_price)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Price {
    pub product_id: i32,
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
}