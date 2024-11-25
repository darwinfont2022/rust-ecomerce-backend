use crate::modules::variation_price::model::domain::variation_price::{VariationPrice, NewVariationPrice, VariationPriceUpdate};
use crate::modules::variations::model::domain::variation::Variation;
use diesel::result::Error as DieselError;
use diesel::prelude::{PgConnection, RunQueryDsl, SelectableHelper, BelongingToDsl, ExpressionMethods, QueryDsl};

impl VariationPrice {
    pub fn save(conn: &mut PgConnection, mut price_new: NewVariationPrice, id_variation: i32) -> Result<Self, DieselError> {
        use crate::schema::variation_price::dsl::*;
        price_new.variation_id = id_variation;

        diesel::insert_into(variation_price)
            .values(&price_new)
            .returning(VariationPrice::as_returning())
            .get_result(conn)
    }

    pub fn find_by_variation(conn: &mut PgConnection, variation: i32) -> Result<Self, DieselError> {
        use crate::schema::variation_price::dsl::*;

        variation_price.select(VariationPrice::as_returning())
            .order_by(create_at.desc())
            .filter(variation_id.eq(variation))
            .first(conn)
    }

    pub fn find_all_by_variation(conn: &mut PgConnection, variation: i32) -> Result<Vec<Self>, DieselError> {
        use crate::schema::variation_price::dsl::*;

        variation_price.select(VariationPrice::as_returning())
            .order_by(price_id.desc())
            .filter(variation_id.eq(variation))
            .load(conn)
    }

    pub fn find_all_by_parents(conn: &mut PgConnection,vtns: &Vec<Variation>) -> Result<Vec<VariationPrice>, DieselError> {
        VariationPrice::belonging_to(vtns)
            .select(VariationPrice::as_select())
            .load(conn)
    }

    pub fn update(conn: &mut PgConnection, id_price: i32, mut price_in: VariationPriceUpdate) -> Result<usize, DieselError> {
        use crate::schema::variation_price::dsl::*;

        price_in.price_id = id_price;

        diesel::update(variation_price)
            .set(&price_in)
            .execute(conn)
    }

    pub fn delete(conn: &mut PgConnection, variation_price_id: &i32) -> Result<usize, DieselError> {
        use crate::schema::variation_price::dsl::*;

        diesel::delete(variation_price.filter(price_id.eq(variation_price_id)))
            .execute(conn)
    }
}