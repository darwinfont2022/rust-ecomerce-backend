use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListingReqDto {
    pub listing_type_id: Option<String>,
    pub listing_source: Option<String>,
    pub catalog_listing: Option<bool>,
}