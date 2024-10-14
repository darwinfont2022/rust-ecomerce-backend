use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::seller_payment_methods)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct  SellerPaymentMethod {
    pub product_id: i32,
    pub method_id: Option<String>,
    pub method_name: Option<String>,
}