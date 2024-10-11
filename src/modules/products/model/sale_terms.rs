use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::sale_terms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SaleTerms {
    pub product_id: i32,
    pub term_id: Option<String>,
    pub term_name: Option<String>,
    pub term_value_id: Option<String>,
    pub term_value_name: Option<String>,
}