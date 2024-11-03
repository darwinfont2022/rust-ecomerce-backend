use diesel::Insertable;
use crate::modules::products::model::dto::product_req_dto::ProductReqDTO;
use crate::schema::products;
#[derive(Insertable, Debug, Clone)]
#[table_name = "products"]
pub struct ProductNew {
    pub mlb_id: String,
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
    pub health: Option<i32>,
    pub date_created: Option<chrono::NaiveDateTime>,
    pub last_updated: Option<chrono::NaiveDateTime>,
}

impl From<ProductReqDTO> for ProductNew {
    fn from(value: ProductReqDTO) -> Self {
        let now = chrono::Utc::now().naive_utc();
        ProductNew {
            mlb_id: value.mlb_id,
            site_id: value.site_id,
            title: Some(value.title),
            category_id: value.category_id,
            official_store_id: value.official_store_id,
            buying_mode: value.buying_mode,
            start_time: value.start_time,
            stop_time: value.stop_time,
            condition: value.condition,
            international_delivery_mode: value.international_delivery_mode,
            status: value.status,
            warranty: value.warranty,
            catalog_product_id: value.catalog_product_id,
            domain_id: value.domain_id,
            parent_item_id: value.parent_item_id,
            automatic_relist: value.automatic_relist,
            health: None,
            date_created: Some(now),
            last_updated: Some(now),
        }
    }
}