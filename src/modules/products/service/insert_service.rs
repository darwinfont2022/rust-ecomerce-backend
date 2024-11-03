use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use crate::modules::products::model::domain::product::{Product};
use crate::modules::products::model::domain::product_new::ProductNew;

impl Product {
    pub fn insert(product_new: ProductNew, conn: &mut PgConnection) -> Result<Product, diesel::result::Error> {
        use crate::schema::products::dsl::*;

        Ok(diesel::insert_into(products)
            .values(product_new)
            .returning(Product::as_returning())
            .get_result(conn)
            .expect("error adding product"))
    }
}