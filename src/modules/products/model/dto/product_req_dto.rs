use serde::{Deserialize};
use serde_valid::Validate;
use crate::modules::products::model::dto::listing::listing_req_dto::ListingReqDto;
use crate::modules::products::model::dto::media::media_req_dto::MediaReqDto;
use crate::modules::products::model::dto::price::price_req_dto::PriceReqDTO;
use crate::modules::products::model::dto::quantity::quantity_req_dto::QuantityReqDto;
use crate::modules::products::model::dto::sale_term::sale_term_req_dto::SaleTermReqDTO;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct ProductReqDTO {
    #[serde(rename = "mlbId")]
    #[validate(max_length = 20)]
    pub mlb_id: String,
    #[validate(max_length= 10)]
    #[serde(rename = "siteId")]
    pub site_id: Option<String>,
    pub title: String,
    pub seller_id: i32,
    #[serde(rename = "categoryId")]
    // #[validate(max_length = 20)]
    pub category_id: Option<String>,
    #[serde(rename = "officialStoreId")]
    pub official_store_id: Option<i32>,
    pub prices: PriceReqDTO,
    pub stock: QuantityReqDto,
    pub sale_terms: Option<Vec<SaleTermReqDTO>>,
    #[serde(rename = "buyingMode")]
    // buy_it_now, auction, classified
    // #[validate(max_length = 20)]
    pub buying_mode: Option<String>,
    pub listing: Option<ListingReqDto>,
    pub media: Option<Vec<MediaReqDto>>,
    #[serde(rename = "startTime", default = "start_time_default")]
    pub start_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "stopTime")]
    pub stop_time: Option<chrono::NaiveDateTime>,
    // Posibles valores: new, used, refurbished, damaged, open-box
    #[validate(max_length = 20)]
    #[serde(default = "condition_default")]
    pub condition: Option<String>,
    #[serde(rename = "deliveryMode", default = "delivery_default")]
    // standard, express, none, free, courier, pick-up
    #[validate(max_length = 20)]
    #[validate(pattern = r"(standard|express|none|free|courier|pick-up)", message = "Invalid delivery mode")]
    pub international_delivery_mode: Option<String>,
    // Disponible, Agotado, Descontinuado, En espera, Reservado
    // En producción, Preventa, Inactivo, Retirado, Bloqueado
    #[serde(default = "status_default")]
    pub status: Option<String>,
    // #[validate(max_length = 20)]
    pub warranty: Option<String>,
    #[serde(rename = "catalogProductId")]
    // #[validate(max_length = 20)]
    pub catalog_product_id: Option<String>,
    #[serde(rename = "domainId")]
    // #[validate(max_length = 20)]
    pub domain_id: Option<String>,
    #[serde(rename = "parentItemId")]
    pub parent_item_id: Option<String>,
    #[serde(rename = "automaticRelist", default = "automatic_relist")]
    pub automatic_relist: Option<bool>,
}
fn delivery_default() -> Option<String> { Some("standard".to_string()) }
fn condition_default() -> Option<String> {
    Some("new".to_string())
}
fn automatic_relist() -> Option<bool> { Some(true) }
fn status_default() -> Option<String> { Some("Disponible".to_string()) }
fn start_time_default() -> Option<chrono::NaiveDateTime> { Some(chrono::Utc::now().naive_utc()) }