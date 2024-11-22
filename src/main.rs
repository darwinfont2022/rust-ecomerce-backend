use actix_web::{web, App, HttpServer};
use modules::products;
use crate::modules::variation_price::model::domain::variation_price::VariationPrice;
use crate::modules::variations::api::VariationApi;

mod api;
mod config;
mod db;
mod modules;
mod schema;
mod exceptions;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let db_connection = db::connection::db_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            .service(products::api::api_products::product_scope())
            .service(VariationApi::scope())
            .service(VariationPrice::api())
    })
        .bind((config::environment::read_setting().server.host, config::environment::read_setting().server.port))?
        .run()
        .await
}
