use actix_web::{post, web, HttpResponse, Responder, Scope};
use crate::db::db_pool::DbPool;
use crate::modules::attribute_combinations::model::domain::attribute_combination::AttributeCombination;
use crate::modules::attribute_combinations::model::dto::attribute_combination_dto::AttributeCombinationReq;

pub fn api() -> Scope {
    web::scope("/attribute-combinations/")
}

#[post("/{variation}")]
pub async fn save_combinations(
    db_pool: web::Data<DbPool>,
    variation: web::Path<i32>,
    combinations: web::Json<Vec<AttributeCombinationReq>>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    match AttributeCombination::save_all_combination(&mut conn, variation.into_inner(), combinations.into_inner()) {
        Ok(combination_saved) => HttpResponse::Ok().json(combination_saved),
        Err(_) => HttpResponse::InternalServerError().json("Something went wrong")
    }
}