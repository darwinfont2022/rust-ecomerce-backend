use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::product_listing)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Listing {
    pub product_id: i32,
    pub listing_type_id: Option<String>,
    pub listing_source: Option<String>,
    pub catalog_listing: Option<bool>,
}