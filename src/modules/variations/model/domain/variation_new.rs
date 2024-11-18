use diesel::{Insertable};
use crate::schema::variations;

#[derive(Insertable, Debug, Clone)]
#[table_name = "variations"]
pub struct VariationNew {
    pub product_id: Option<i32>,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub catalog_product_id: Option<String>,
}
