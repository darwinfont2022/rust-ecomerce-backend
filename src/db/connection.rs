use std::process::exit;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool}
};

use log::{error, info};
use crate::config::environment;
use crate::config::setting::Setting;
use crate::db::db_pool::DbPool;

pub fn db_connection() -> DbPool {
    let config: Setting = environment::read_setting();
    let initial_url = "postgres://user:password@db_host/name".to_string();
    let url = initial_url
        .replace("user", config.persistence.user.as_str())
        .replace("password", config.persistence.password.as_str())
        .replace("db_host", config.persistence.host.as_str())
        .replace("name", config.persistence.name.as_str());

    info!("Connecting to {}", &url);
    let connection = ConnectionManager::<PgConnection>::new(url);
    info!("Connection manager created");

    match Pool::builder()
        .build(connection) {
        Ok(pool) => return pool,
        Err(_) => {
            error!("Error creating connection pool");
            exit(2)
        }
    }

}