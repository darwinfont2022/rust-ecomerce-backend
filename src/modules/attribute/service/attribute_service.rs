use crate::modules::attribute::domain::attribute::{Attribute, NewAttribute, AttributeUpdate};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, RunQueryDsl, SelectableHelper};
use diesel::result::Error as DieselError;
impl Attribute {
    pub fn inserts(conn: &mut PgConnection, attributes_in: Vec<NewAttribute>) -> Result<Vec<Self>, DieselError> {
        use crate::schema::attributes::dsl::*;

        diesel::insert_into(attributes)
            .values(&attributes_in)
            .returning(Attribute::as_returning())
            .get_results(conn)
    }

    pub fn find(conn: &mut PgConnection, id: &i32) -> Result<Self, DieselError> {
        use crate::schema::attributes::dsl::*;

        attributes
            .select(Attribute::as_select())
            .find(id)
            .get_result(conn)
    }

    pub fn find_all_by_product_id(conn: &mut PgConnection, id_product: i32 ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::attributes::dsl::*;

        attributes
            .select(Attribute::as_select())
            .filter(product_id.eq(id_product))
            .get_results(conn)
    }

    pub fn update(conn: &mut PgConnection, id: i32, mut attribute_in: AttributeUpdate) -> Result<Self, DieselError> {
        use crate::schema::attributes::dsl::*;
        attribute_in.attribute_id = id;

        diesel::update(attributes.find(id))
            .set(&attribute_in)
            .returning(Attribute::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, id: &i32) -> Result<usize, DieselError> {
        use crate::schema::attributes::dsl::*;
        diesel::delete(attributes.find(id)).execute(conn)
    }

    pub fn delete_all_by_product_id(conn: &mut PgConnection, id: &i32 ) -> Result<usize, DieselError> {
        use crate::schema::attributes::dsl::*;
        diesel::delete(attributes.filter(product_id.eq(id)))
            .execute(conn)
    }
}