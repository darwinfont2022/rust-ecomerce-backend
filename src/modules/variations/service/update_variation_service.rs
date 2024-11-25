use diesel::{PgConnection, RunQueryDsl, ExpressionMethods, SelectableHelper};
use diesel::result::Error as DieselError;
use crate::modules::variations::model::domain::variation::Variation;
use crate::modules::variations::model::dto::variation_dto::VariationDto;
use crate::modules::variations::model::domain::variation_update::VariationUpdateModel;

impl Variation {
    pub(crate) fn update(conn: &mut PgConnection, id: i32, dto: VariationDto) -> Result<Self, DieselError> {
        use crate::schema::variations::dsl::*;
        let mut variation: VariationUpdateModel = VariationUpdateModel::from(dto);
        variation.variation_id = id;
        diesel::update(variations)
            .set(variation)
            .returning(Variation::as_returning())
            .get_result(conn)
    }
}