use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use crate::modules::products::model::domain::product::Product;
use diesel::ExpressionMethods;
impl Product {
    pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::products::dsl::*;

        diesel::delete(
            products.filter(product_id.eq(id))
        ).execute(conn)
    }
}