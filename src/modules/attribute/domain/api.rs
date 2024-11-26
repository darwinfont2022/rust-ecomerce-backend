use crate::modules::attribute::domain::attribute::{Attribute, AttributeUpdate, NewAttribute};
use actix_web::{Scope, HttpResponse, web, post, Responder, get, delete, put};
use crate::db::db_pool::DbPool;
use crate::exceptions::exception::ApiException;
use crate::modules::attribute::dto::attribute_dto::AttributeDtoReq;

impl Attribute {
    pub fn api() -> Scope {
        web::scope("/api/attributes")
            .service(save)
            .service(find_attribute)
            .service(find_by_product_id)
            .service(update)
            .service(delete_attr)
            .service(delete_by_product)
    }
}
#[post("")]
async fn save(
    db_pool: web::Data<DbPool>,
    data: web::Json<Vec<AttributeDtoReq>>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("Could not get db connection from pool");
    let data = data.into_inner();

    match Attribute::inserts(&mut conn, AttributeDtoReq::map_attributes(data)) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => ApiException::bad_request("bad request, error in attributes", None)
    }
}

#[get("/{attribute_id}")]
async fn find_attribute(
    db_pool: web::Data<DbPool>,
    id_attribute: web::Path<i32>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("Could not get db connection from pool");
    let attribute_id = id_attribute.into_inner();

    match Attribute::find(&mut conn, &attribute_id) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => ApiException::not_found(format!(" attributes by id {}", &attribute_id).as_str(), None)
    }
}

#[get("/product/{product_id}")]
async fn find_by_product_id(
    db_pool: web::Data<DbPool>,
    id_product: web::Path<i32>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("Could not get db connection from pool");
    let product_id = id_product.into_inner();

    match Attribute::find_all_by_product_id(&mut conn, &product_id) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => ApiException::not_found(format!(" attributes by product id {}", &product_id).as_str(), None)
    }
}

#[delete("/{attribute_id}")]
async fn delete_attr(
    db_pool: web::Data<DbPool>,
    attribute_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("Could not get db connection from pool");
    let attribute_id = attribute_id.into_inner();
    match Attribute::delete_all_by_product_id(&mut conn, &attribute_id) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => ApiException::not_found(format!(" attribute with id {}", attribute_id).as_str(), None)
    }
}

#[put("/{attribute_id}")]
async fn update(
    db_pool: web::Data<DbPool>,
    attribute_id: web::Path<i32>,
    data: web::Json<AttributeDtoReq>
) -> impl Responder {
    let mut conn = db_pool.get().expect("Could not get db connection from pool");
    let attribute_id = attribute_id.into_inner();
    let data = data.into_inner();

    match Attribute::update(&mut conn, attribute_id, data.into()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => ApiException::not_found(format!("attributes by id {}", &attribute_id).as_str(), None)
    }
}

#[delete("/product/{product_id}")]
async fn delete_by_product(
    db_pool: web::Data<DbPool>,
    id_product: web::Path<i32>,
) -> impl Responder {
    let mut conn = db_pool.get().expect("Could not get db connection from pool");
    let product_id = id_product.into_inner();
    match Attribute::delete_all_by_product_id(&mut conn, &product_id) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => ApiException::bad_request("bad request, error in attributes", None)
    }
}