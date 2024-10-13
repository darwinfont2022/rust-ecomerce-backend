use actix_web::{web, App, HttpServer};
use log::info;

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

    info!("{:?}", config::environment::read_setting());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            .service(api::products::product_scope())
    })
        .bind((config::environment::read_setting().server.host, config::environment::read_setting().server.port))?
        .run()
        .await
}
