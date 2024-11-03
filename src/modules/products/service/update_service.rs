use diesel::{PgConnection, RunQueryDsl};
use crate::modules::products::model::domain::product::Product;
use crate::modules::products::model::domain::product_update::ProductUpdate;

impl Product {
    pub fn update(conn: &mut PgConnection, update_dto: ProductUpdate) -> Result<usize, diesel::result::Error> {
        use crate::schema::products::dsl::*;

        diesel::update(products)
            .set(update_dto)
            .execute(conn)
    }
}