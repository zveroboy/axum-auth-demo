use axum::extract::FromRef;

use super::config::Config;
use super::store::Db;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
    pub db: Db,
}
