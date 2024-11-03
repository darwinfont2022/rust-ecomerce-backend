use serde::{Serialize};
use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use crate::modules::products::model::dto::listing::listing_res_dto::ListingResDto;
use crate::modules::products::model::dto::media::media_res_dto::MediaResDto;
use crate::modules::products::model::dto::price::price_res_dto::PriceResDTO;
use crate::modules::products::model::dto::quantity::quantity_res_dto::QuantityResDto;
use crate::modules::products::model::dto::sale_term::sale_term_res_dto::SaleTermResDTO;

#[derive(Debug, Serialize, Clone)]
pub struct ProductResDTO {
    #[serde(rename = "mlbId")]
    pub mlb_id: String,
    #[serde(rename = "siteId")]
    pub site_id: Option<String>,
    pub title: String,
    pub seller_id: String,
    #[serde(rename = "categoryId")]
    pub category_id: Option<String>,
    #[serde(rename = "officialStoreId")]
    pub official_store_id: Option<i32>,
    pub prices: PriceResDTO,
    pub stock: QuantityResDto,
    pub sale_terms: Option<Vec<SaleTermResDTO>>,
    #[serde(rename = "buyingMode")]
    // buy_it_now, auction, classified
    pub buying_mode: Option<String>,
    pub listing: Option<ListingResDto>,
    pub media: Option<Vec<MediaResDto>>,
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "stopTime")]
    pub stop_time: Option<chrono::NaiveDateTime>,
    // Posibles valores: new, used, refurbished, damaged, open-box
    pub condition: Option<String>,
    #[serde(rename = "internationalDeliveryMode")]
    // standard, express, none, free, courier, pick-up
    pub international_delivery_mode: Option<String>,
    // Disponible, Agotado, Descontinuado, En espera, Reservado
    // En producción, Preventa, Inactivo, Retirado, Bloqueado
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

impl Responder for ProductResDTO {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}