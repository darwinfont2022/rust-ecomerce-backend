use diesel::{AsChangeset, Identifiable, Queryable};
use crate::schema::variations;
#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = variations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(variation_id))]
pub struct VariationUpdateModel {
    pub product_id: Option<i32>,
    pub variation_id: i32,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub catalog_product_id: Option<String>,
}