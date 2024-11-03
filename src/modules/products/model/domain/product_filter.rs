use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductFilter {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub id: Option<i32>,
    pub mlb_id: Option<String>,
    pub site_id: Option<String>,
    pub title: Option<String>,
    pub category_id: Option<String>,
    pub official_store_id: Option<i32>,
    pub buying_mode: Option<String>,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub stop_time: Option<chrono::NaiveDateTime>,
    pub condition: Option<String>,
    pub international_delivery_mode: Option<String>,
    pub status: Option<String>,
    pub warranty: Option<String>,
    pub catalog_product_id: Option<String>,
    pub domain_id: Option<String>,
    pub parent_item_id: Option<String>,
    pub automatic_relist: Option<bool>,
    pub date_created: Option<chrono::NaiveDateTime>,
    pub last_updated: Option<chrono::NaiveDateTime>,
    pub health: Option<i32>,
}