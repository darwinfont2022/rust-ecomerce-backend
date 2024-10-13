use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::product_inventory)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Inventory {
    pub initial_quantity: Option<i32>,
    pub available_quantity: Option<i32>,
    pub sold_quantity: Option<i32>,
}