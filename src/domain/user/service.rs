use async_trait::async_trait;

use super::entity::User;
use super::error::Result;
use super::repository::UserRepository;

pub struct LoginParams {
    email: String,
    password: String,
}

pub struct RegisterParams {
    email: String,
    password: String,
}

#[async_trait]
pub trait UserCommands {
    async fn login(&mut self, params: LoginParams) -> Result<bool>;

    async fn register(&self, params: RegisterParams) -> Result<i64>;

    // async fn confirm(&mut self, id: u32) -> Result<()>;
}

#[derive(Clone)]
pub struct UserService<Repo> {
    repository: Repo,
}

impl<Repo: UserRepository> UserService<Repo> {
    pub fn new(repository: Repo) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<Repo: UserRepository> UserCommands for UserService<Repo>
where
    Repo: UserRepository,
{
    async fn login(&mut self, params: LoginParams) -> Result<bool> {
        Ok(true)
    }

    async fn register(&self, params: RegisterParams) -> Result<i64> {
        Ok(123)
    }
}
