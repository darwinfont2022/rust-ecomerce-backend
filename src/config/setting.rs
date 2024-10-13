use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Setting {
    pub server: ServerSetting,
    pub persistence: DatabaseSetting,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerSetting {
    pub port: u16,
    pub host: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseSetting {
    pub name: String,
    pub host: String,
    pub user: String,
    pub password: String,
}