use axum::extract::FromRef;

use super::store::Db;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub foo: String,
    pub db: Db,
}
