use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use diesel::result::Error as DieselError;
use diesel::SelectableHelper;
use diesel::ExpressionMethods;
use crate::modules::variations::model::domain::variation::Variation;

impl Variation {
    pub fn find_variation_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Vec<Variation>> {
        use crate::schema::variations::dsl::*;

        variations.select(Variation::as_select()).find(id).load::<Variation>(conn)
    }

    pub fn find_all_by_product_id(conn: &mut PgConnection, id: i32) -> Result<Vec<Variation>, DieselError> {
        use crate::schema::variations::dsl::*;

        variations.filter(product_id.eq(id)).select(Variation::as_select()).load::<Variation>(conn)
    }
}