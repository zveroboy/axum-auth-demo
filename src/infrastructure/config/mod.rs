mod db;
mod util;

use self::db::DbConfig;
use self::util::get_env_required;
use std::sync::OnceLock;

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::load_from_env)
}

pub struct Config {
    pub db: DbConfig,
    pub web_folder: String,
}

impl Config {
    pub fn load_from_env() -> Self {
        Self {
            web_folder: get_env_required("SERVICE_WEB_FOLDER"),
            db: DbConfig::new(),
        }
    }
}
