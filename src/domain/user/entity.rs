use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
}
