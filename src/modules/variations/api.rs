use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use crate::db::db_pool::DbPool;
use crate::modules::variations::model::domain::variation::Variation;
use crate::modules::variations::model::dto::variation_dto::VariationDto;

pub struct VariationApi {}

impl VariationApi {
    pub fn scope() -> Scope {
        web::scope("/api/variations")
            .service(save_variation)
            .service(find_variation_by_id)
            .service(find_all_by_product_id)
            .service(delete_by_id)
    }
}

#[post("")]
async fn save_variation(
    db_pool: web::Data<DbPool>,
    variation_dto: web::Json<VariationDto>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");
    let variation_dto: VariationDto = variation_dto.into_inner();
    match Variation::save_full_variation(&mut conn, variation_dto.into()) {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{id}")]
async fn find_variation_by_id(
    db_pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    match Variation::find_variation_by_id(&mut conn, id.into_inner()) {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/product/{product_id}")]
async fn find_all_by_product_id(
    db_pool: web::Data<DbPool>,
    product_id: web::Path<i32>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    match Variation::find_all_by_product_id(&mut conn, product_id.into_inner()) {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{id}")]
async fn delete_by_id(
    db_pool: web::Data<DbPool>,
    id: web::Path<i32>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    match Variation::delete_variation(&mut conn, id.into_inner()) {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(_) => HttpResponse::InternalServerError().body("delete by id"),
    }
}