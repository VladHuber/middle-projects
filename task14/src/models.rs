use serde::{Deserialize, Serialize};

use crate::error::AppResult;
use crate::validator;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub port: u16,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub debug: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
}

impl Config {
    pub fn set_value(&mut self, key: &str, value: &str) -> AppResult<()> {
        match key {
            "app_name" => self.app_name = value.to_string(),
            "port" => {
                let port = value.parse::<u16>().map_err(|_| {
                    format!("Ошибка: Значение '{value}' вне диапазона портов (1-65535)")
                })?;

                let valid_port = validator::check_port(port)?;
                self.port = valid_port;
            }
            "debug" => self.debug = value.parse::<bool>().map_err(|_| "debug должен быть true или false")?,
            "database.host" => self.database.host = value.to_string(),
            "database.port" => {
                let port = value.parse::<u16>().map_err(|_| {
                    format!("Ошибка: Значение '{value}' вне диапазона портов (1-65535)")
                })?;

                let valid_port = validator::check_port(port)?;
                self.database.port = valid_port;
            }
            "database.user" => self.database.user = value.to_string(),
            "database.password" => self.database.password = value.to_string(),
            "logging.level" => self.logging.level = value.to_string(),
            "logging.file" => self.logging.file = value.to_string(),
            _ => { return Err(format!("Неизвестный ключ: {}", key).into());}
        }
        Ok(())
    }
}
