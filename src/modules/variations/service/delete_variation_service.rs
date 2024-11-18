use diesel::{PgConnection, RunQueryDsl};
use diesel::result::Error as DieselError;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::modules::variations::model::domain::variation::Variation;

impl Variation {
    pub fn delete_variation(conn: &mut PgConnection, id: i32) -> Result<usize, DieselError> {
        use crate::schema::variations::dsl::*;

        diesel::delete(variations.filter(variation_id.eq(id)))
            .execute(conn)
    }

    pub fn delete_variations(conn: &mut PgConnection, ids: Vec<i32>) -> Result<usize, DieselError> {
        use crate::schema::variations::dsl::*;

        diesel::delete(variations.filter(variation_id.eq_any(ids))).execute(conn)
    }
}