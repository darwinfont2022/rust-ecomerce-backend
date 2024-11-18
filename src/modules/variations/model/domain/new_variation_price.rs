use bigdecimal::BigDecimal;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use crate::modules::variations::model::domain::variation::Variation;

#[derive(Queryable, Selectable, Identifiable, Insertable ,Associations, Clone, Debug, PartialEq)]
#[diesel(belongs_to(Variation))]
#[diesel(table_name = crate::schema::variation_price)]
#[diesel(primary_key(variation_id))]
pub struct NewVariationPrice {
    pub variation_id: i32,
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
    pub price_type: Option<String>,
    pub create_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, AsChangeset, Debug, PartialEq)]
#[diesel(belongs_to(Variation))]
#[diesel(table_name = crate::schema::variation_price)]
#[diesel(primary_key(price_id))]
pub struct VariationPriceUpdate {
    pub price_id: i32,
    pub variation_id: i32,
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
    pub price_type: Option<String>,
    pub update_at: Option<chrono::NaiveDateTime>,
}