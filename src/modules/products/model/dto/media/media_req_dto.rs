use serde::{Deserialize, Serialize};
use crate::modules::products::model::dto::media::picture_req_dto::PictureReqDto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaReqDto {
    pub permalink: Option<String>,
    pub thumbnail_id: Option<String>,
    pub thumbnail: Option<String>,
    pub video_id: Option<String>,
    pub pictures: Option<Vec<PictureReqDto>>,
}