use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct AttributeDtoReq {
    pub product_id: Option<i32>,
    pub attribute_name: Option<String>,
    pub value_id: Option<String>,
    pub value_name: Option<String>,
    pub attribute_group_id: Option<String>,
    pub attribute_group_name: Option<String>,
    pub value_type: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AttributeDtoRes {
    pub attribute_id: i32,
    pub product_id: Option<i32>,
    pub attribute_name: Option<String>,
    pub value_id: Option<String>,
    pub value_name: Option<String>,
    pub attribute_group_id: Option<String>,
    pub attribute_group_name: Option<String>,
    pub value_type: Option<String>,
    pub created_ad: Option<chrono::NaiveDateTime>,
    pub updated_ad: Option<chrono::NaiveDateTime>,
}