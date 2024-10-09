use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[post("/products")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Create Products")
}

#[get("/products")]
pub async fn find() -> impl Responder {
    HttpResponse::Ok().body("Get Products")
}

#[put("/products")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Update Products")
}

#[delete("/products")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Delete Products")
}