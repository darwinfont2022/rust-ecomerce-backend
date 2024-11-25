use diesel::{Identifiable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Identifiable, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::variations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(variation_id))]
pub struct Variation {
    pub variation_id: i32,
    pub product_id: Option<i32>,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
    pub catalog_product_id: Option<String>,
}

impl Variation {
    pub fn new() -> Self{
        Variation{
            variation_id: 0,
            product_id: Option::from(0),
            available_quantity: None,
            sold_quantity: None,
            catalog_product_id: None,
        }
    }
}