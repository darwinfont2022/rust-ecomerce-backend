use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaReqDto {
    pub permalink: String,
    pub thumbnail_id: String,
    pub thumbnail: String,
}