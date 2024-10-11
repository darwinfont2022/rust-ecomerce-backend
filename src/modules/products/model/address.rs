use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::seller_address)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SellerAddress {
    pub product_id: i32,
    pub city_id: Option<String>,
    pub city_name: Option<String>,
    pub state_id: Option<String>,
    pub state_name: Option<String>,
    pub country_id: Option<String>,
    pub country_name: Option<String>,
    pub neighborhood_id: Option<String>,
    pub neighborhood_name: Option<String>,
}