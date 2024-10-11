use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::product_media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub product_id: i32,
    pub permalink: Option<String>,
    pub thumbnail_id: Option<String>,
    pub thumbnail: Option<String>,
}