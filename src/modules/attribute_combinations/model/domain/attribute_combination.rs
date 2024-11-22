use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::modules::variations::model::domain::variation::Variation;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Clone,Debug)]
#[diesel(table_name = crate::schema::attribute_combinations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(combination_id))]
#[diesel(belongs_to(Variation))]
#[diesel()]
pub struct AttributeCombination {
    pub combination_id: i32,
    pub variation_id: i32,
    pub combination_external_id: Option<String>,
    pub combination_name: Option<String>,
    pub combination_value_id: Option<String>,
    pub combination_value_name: Option<String>,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::attribute_combinations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(combination_id))]
pub struct AttributeCombinationUpdate <'a> {
    pub combination_id : &'a i32,
    pub variation_id : Option<&'a i32>,
    pub combination_external_id : Option<&'a str>,
    pub combination_name : Option<&'a str>,
    pub combination_value_id : Option<&'a str>,
    pub combination_value_name : Option<&'a str>,
}

impl Responder for AttributeCombination {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}