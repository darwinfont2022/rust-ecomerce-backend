use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use crate::db::db_pool::DbPool;
use crate::exceptions::exception::ApiException;
use crate::modules::attribute_combinations::model::domain::attribute_combination::AttributeCombination;
use crate::modules::attribute_combinations::model::dto::attribute_combination_dto::AttributeCombinationReq;
use crate::schema::variations::variation_id;

impl AttributeCombination {
    pub fn api() -> Scope {
        web::scope("/api/attribute-combinations")
            .service(save_combinations)
            .service(find)
            .service(update_combination)
            .service(delete_combination)
    }
}

#[post("/{variation}")]
pub async fn save_combinations(
    db_pool: web::Data<DbPool>,
    variation: web::Path<i32>,
    combinations: web::Json<Vec<AttributeCombinationReq>>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");
    let variation = variation.into_inner();

    match AttributeCombination::save_all_combination(&mut conn, variation.clone(), combinations.into_inner()) {
        Ok(combination_saved) => HttpResponse::Ok().json(combination_saved),
        Err(_) => ApiException::not_found(format!("Attributes combinations by variation id {}", variation).as_str(), None)
    }
}

#[get("/{id}")]
async fn find(db_pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");
    let id = id.into_inner();
    match AttributeCombination::find_combination_by_variation_id(&mut conn, id.clone()) {
        Ok(combinations) => HttpResponse::Ok().json(combinations),
        Err(_) => ApiException::not_found(format!("Attributes combinations by variation id {}", id).as_str(), None)
    }
}

#[put("/{combination_id}")]
async fn update_combination(
    db_pool: web::Data<DbPool>,
    combination_id: web::Path<i32>,
    combination_dto: web::Json<AttributeCombinationReq>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");
    let combination_dto = combination_dto.into_inner();
    let combination_id = combination_id.into_inner();

    match AttributeCombination::update(&mut conn, combination_id.clone(), combination_dto) {
        Ok(updated_combination) => HttpResponse::Ok().json(updated_combination),
        Err(_) => ApiException::not_found(format!("Attribute combination by combination id {}", combination_id).as_str(), None)
    }
}

#[delete("/{combination_id}")]
async fn delete_combination(
    db_pool: web::Data<DbPool>,
    combination_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    let combination_id = combination_id.into_inner();
    match AttributeCombination::delete(&mut conn, combination_id.clone()) {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(_) => ApiException::not_found(format!("Attribute combination by combination id {}", combination_id).as_str(), None)
    }
}