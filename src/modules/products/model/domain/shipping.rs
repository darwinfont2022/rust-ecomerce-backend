use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::shipping)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Shipping {
    pub product_id: i32,
    pub mode: Option<String>,
    pub free_shipping: Option<bool>,
    pub logistic_type: Option<String>,
    // pub method: Option<String>,
    // pub dimensions: Option<String>,
    // pub local_pick_up: Option<bool>,
    // pub store_pick_up: Option<bool>,
}