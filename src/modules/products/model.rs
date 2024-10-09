use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub product_id: i32,
    pub mlb_id: String,
    pub site_id: Option<String>,
    pub title: Option<String>,
    pub category_id: Option<String>,
    pub official_store_id: Option<i32>,
    pub price: Option<BigDecimal>,
    pub base_price: Option<BigDecimal>,
    pub original_price: Option<BigDecimal>,
    pub currency_id: Option<String>,
    pub initial_quantity: Option<i32>,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub buying_mode: Option<String>,
    pub listing_type_id: Option<String>,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub stop_time: Option<chrono::NaiveDateTime>,
    pub condition: Option<String>,
    pub permalink: Option<String>,
    pub thumbnail_id: Option<String>,
    pub thumbnail: Option<String>,
    pub international_delivery_mode: Option<String>,
    pub listing_source: Option<String>,
    pub status: Option<String>,
    pub warranty: Option<String>,
    pub catalog_product_id: Option<String>,
    pub domain_id: Option<String>,
    pub parent_item_id: Option<String>,
    pub automatic_relist: Option<bool>,
    pub date_created: Option<chrono::NaiveDateTime>,
    pub last_updated: Option<chrono::NaiveDateTime>,
    pub health: Option<i32>,
    pub catalog_listing: Option<bool>,
}