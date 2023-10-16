use async_trait::async_trait;
use scrypt::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString,
    },
    Scrypt,
};

use super::error::{Error, Result};
use super::repository::UserRepository;
use super::{entity::User, repository::CreateParams};

pub struct LoginParams {
    pub email: String,
    pub password: String,
}

pub struct RegisterParams {
    pub email: String,
    pub password: String,
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
    async fn login(
        &mut self,
        LoginParams {
            email,
            password: pure_password,
        }: LoginParams,
    ) -> Result<bool> {
        // let user = self
        //     .repository
        //     .find_by_email(email)
        //     .await
        //     .map_err(|_| Error::FailToLogin)?;

        // let pure_password_bytes = pure_password.as_bytes();
        // let (salt, stored_password) = user.password.split_at(Salt::RECOMMENDED_LENGTH);

        // let password_hash = Scrypt
        //     .hash_password(pure_password_bytes, salt.into())
        //     .map_err(|_| Error::FailedToBuildPasswordHash)?
        //     .to_string();

        // let stored_password_hash =
        //     PasswordHash::new(&password_hash).map_err(|_| Error::FailedToBuildPasswordHash)?;
        // Ok(Scrypt
        //     .verify_password(pure_password_bytes, &stored_password_hash)
        //     .is_ok())

        Ok(true)
    }

    async fn register(&self, RegisterParams { email, password }: RegisterParams) -> Result<i64> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| Error::FailedToBuildPasswordHash)?
            .to_string();

        self.repository
            .create(CreateParams {
                email,
                password: password_hash,
            })
            .await
    }
}
