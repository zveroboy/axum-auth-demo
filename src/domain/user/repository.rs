use async_trait::async_trait;

use super::entity::User;
use super::error::Result;

pub struct CreateParams {
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait UserRepository: Sync + Send + Clone {
    async fn create(&self, params: CreateParams) -> Result<i64>;
    // async fn find_by_email<P: AsRef<str>>(&self, email: P) -> Result<User>;
}
