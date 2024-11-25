use diesel::{Queryable, Selectable, Associations, Insertable, AsChangeset, Identifiable};
use serde::Serialize;
use crate::modules::products::model::domain::product::Product;
#[derive(Queryable, Selectable, Associations, Serialize, Debug)]
#[diesel(table_name = crate::schema::attributes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(attribute_id))]
#[diesel(belongs_to(Product))]
pub struct Attribute {
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
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::attributes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAttribute {
    pub product_id: Option<i32>,
    pub attribute_name: Option<String>,
    pub value_id: Option<String>,
    pub value_name: Option<String>,
    pub attribute_group_id: Option<String>,
    pub attribute_group_name: Option<String>,
    pub value_type: Option<String>,
    pub created_ad: chrono::NaiveDateTime,
}
#[derive(Identifiable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::attributes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(attribute_id))]
pub struct AttributeUpdate {
    pub attribute_id: i32,
    pub product_id: Option<i32>,
    pub attribute_name: Option<String>,
    pub value_id: Option<String>,
    pub value_name: Option<String>,
    pub attribute_group_id: Option<String>,
    pub attribute_group_name: Option<String>,
    pub value_type: Option<String>,
    pub updated_ad: chrono::NaiveDateTime,
}
