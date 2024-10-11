use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::pictures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Picture {
    pub product_id: i32,
    pub picture_id: Option<String>,
    pub url: Option<String>,
    pub source_url: Option<String>,
    pub size: Option<String>,
    pub max_size: Option<String>,
    pub quality: Option<String>,
}