use async_trait::async_trait;

use super::{entity::User, error::Result};

pub struct CreateParams {
    email: String,
    password: String,
}

#[async_trait]
pub trait UserRepository: Sync + Send + Clone {
    async fn create(&self, params: CreateParams) -> Result<i64>;
    async fn get(&self, params: u64) -> Result<User>;
}
