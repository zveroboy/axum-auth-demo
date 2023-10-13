use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}