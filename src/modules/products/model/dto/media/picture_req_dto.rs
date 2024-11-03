use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PictureReqDto {
    pub url: Option<String>,
    pub source_url: Option<String>,
    pub size: Option<String>,
    pub max_size: Option<String>,
    pub quality: Option<String>,
}