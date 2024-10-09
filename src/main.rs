use actix_web::{App, HttpServer};

mod api;
mod config;
mod db;
mod modules;
mod schema;
mod exceptions;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .service(api::products::create)
            .service(api::products::find)
            .service(api::products::update)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
