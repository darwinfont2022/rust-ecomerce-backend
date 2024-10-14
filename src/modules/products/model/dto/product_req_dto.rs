use serde::{Deserialize, Serialize};
use crate::modules::products::model::dto::listing::listing_req_dto::ListingReqDto;
use crate::modules::products::model::dto::price::price_req_dto::PriceReqDTO;
use crate::modules::products::model::dto::quantity::quantity_req_dto::QuantityReqDto;
use crate::modules::products::model::dto::sale_term::sale_term_req_dto::SaleTermReqDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductReqDTO {
    #[serde(rename = "meliId")]
    pub mlb_id: String,
    #[serde(rename = "siteId")]
    pub site_id: Option<String>,
    pub title: String,
    pub seller_id: String,
    #[serde(rename = "categoryId")]
    pub category_id: Option<String>,
    #[serde(rename = "officialStoreId")]
    pub official_store_id: Option<i32>,
    pub price: PriceReqDTO,
    pub quantity: QuantityReqDto,
    pub sale_terms: Vec<SaleTermReqDTO>,
    #[serde(rename = "buyingMode")]
    // buy_it_now, auction, classified
    pub buying_mode: Option<String>,
    pub listing: Option<ListingReqDto>,
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "stopTime")]
    pub stop_time: Option<chrono::NaiveDateTime>,
    // Posibles valores: new, used, refurbished, damaged, open-box
    pub condition: Option<String>,
    #[serde(rename = "internationalDeliveryMode")]
    pub international_delivery_mode: Option<String>,
    pub status: Option<String>,
    pub warranty: Option<String>,
    #[serde(rename = "catalogProductId")]
    pub catalog_product_id: Option<String>,
    #[serde(rename = "domainId")]
    pub domain_id: Option<String>,
    #[serde(rename = "parentItemId")]
    pub parent_item_id: Option<String>,
    #[serde(rename = "automaticRelist")]
    pub automatic_relist: Option<bool>,
}