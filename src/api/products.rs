use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use crate::db::db_pool::DbPool;
use crate::modules::products::model::dto::product_req_dto::ProductReqDTO;

pub fn product_scope() -> Scope {
  web::scope("/products")
      .service(create_product)
      .service(find_products)
      .service(update_product)
      .service(delete_product)
}

#[post("")]
async fn create_product(
    db_pool: web::Data<DbPool>,
    product_dto: web::Json<ProductReqDTO>
) -> impl Responder {
    HttpResponse::Ok().body("Create Products")
}

#[get("")]
async fn find_products() -> impl Responder {
    HttpResponse::Ok().body("Get Products")
}

#[put("")]
async fn update_product() -> impl Responder {
    HttpResponse::Ok().body("Update Products")
}

#[delete("")]
async fn delete_product() -> impl Responder {
    HttpResponse::Ok().body("Delete Products")
}