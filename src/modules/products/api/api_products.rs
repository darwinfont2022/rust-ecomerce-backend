use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use crate::db::db_pool::DbPool;
use crate::modules::products::model::domain::product::Product;
use crate::modules::products::model::domain::product_filter::ProductFilter;
use crate::modules::products::model::domain::product_new::ProductNew;
use crate::modules::products::model::domain::product_update::ProductUpdate;
use crate::modules::products::model::dto::product_req_dto::ProductReqDTO;

pub fn product_scope() -> Scope {
  web::scope("/api/products")
      .service(create_product)
      .service(find_by_id)
      .service(find_products)
      .service(update_product)
      .service(delete_product)
}

#[post("")]
async fn create_product(
    db_pool: web::Data<DbPool>,
    product_dto: web::Json<ProductReqDTO>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    match Product::insert(ProductNew::from(product_dto.clone()),&mut conn) {
        Ok(rsp) => HttpResponse::Ok().json(rsp),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
#[get("/{id}")]
async fn find_by_id(id: web::Path<i32>, db_pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    let rsp = Product::find_by_id(&mut conn, id.into_inner());

    match rsp {
        Ok(rsp) => HttpResponse::Ok().json(rsp),
        Err(_) => HttpResponse::NotFound().finish()
    }
}
#[get("")]
async fn find_products(
    db_pool: web::Data<DbPool>,
    product_filter: web::Query<ProductFilter>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    HttpResponse::Ok().json(Product::filter(&mut conn, product_filter.into_inner()))
}

#[put("/{id}")]
async fn update_product(
    db_pool: web::Data<DbPool>,
    id: web::Path<i32>,
    product_dto: web::Json<ProductReqDTO>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    match Product::update(&mut conn, ProductUpdate::from(id.into_inner(), product_dto.into_inner())) {
        Ok(rsp) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[delete("/{id}")]
async fn delete_product(
    id: web::Path<i32>,
    db_pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");

    match Product::delete(&mut conn, id.into_inner()) {
        Ok(rsp) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish()
    }
}