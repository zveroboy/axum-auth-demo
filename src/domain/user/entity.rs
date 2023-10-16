use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}
