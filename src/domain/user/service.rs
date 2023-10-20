use std::time::Instant;

use futures::Future;
// use password_auth::{generate_hash, verify_password};
use scrypt::{
    password_hash::{rand_core::OsRng, rand_core::RngCore, Salt},
    scrypt, Params,
};
use subtle::ConstantTimeEq;
use tracing::debug;

use crate::utils::{hex, hex_literal};

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

pub trait UserCommands {
    fn login(&self, params: LoginParams) -> impl Future<Output = Result<bool>> + Send;

    fn register(&self, params: RegisterParams) -> impl Future<Output = Result<i64>> + Send;

    // async fn confirm(&mut self, id: u32) -> Result<()>;
}

const SALT_LENGTH: usize = Salt::RECOMMENDED_LENGTH;
const HASH_LENGTH: usize = Params::RECOMMENDED_LEN;

fn generate_salt() -> [u8; SALT_LENGTH] {
    let mut salt_bytes = [0u8; SALT_LENGTH];
    let mut rng = OsRng;
    rng.fill_bytes(&mut salt_bytes);
    return salt_bytes;
}

fn hash_password(password: &[u8], salt: &[u8]) -> [u8; SALT_LENGTH + HASH_LENGTH] {
    let start = Instant::now();
    let mut password_bytes = [0u8; HASH_LENGTH];

    // In dev mode tuning of opt-level affect the speed of hashing
    let params = Params::new(14, 8, 1, password_bytes.len()).unwrap();

    // This is running on a thread where blocking is fine.
    scrypt(password, salt, &params, &mut password_bytes).unwrap();

    let after_scrypt = start.elapsed();

    let hashed_bytes = {
        let mut bytes = [0u8; SALT_LENGTH + HASH_LENGTH];
        let (one, two) = bytes.split_at_mut(SALT_LENGTH);
        one.copy_from_slice(&salt);
        two.copy_from_slice(&password_bytes);
        bytes
    };

    return hashed_bytes;
}

fn verify_password(pure_password_bytes: &[u8], stored_hashed_bytes: &[u8]) -> bool {
    let (salt_bytes, _password_bytes) = stored_hashed_bytes.split_at(SALT_LENGTH);

    let hashed_bytes = hash_password(pure_password_bytes, &salt_bytes);

    hashed_bytes.ct_eq(stored_hashed_bytes).into()
}

#[derive(Clone)]
pub struct UserService<Repo> {
    repository: Repo,
}

impl<Repo> UserService<Repo> {
    pub fn new(repository: Repo) -> Self {
        Self { repository }
    }
}

impl<Repo: UserRepository> UserCommands for UserService<Repo>
where
    Repo: UserRepository,
{
    async fn login(&self, LoginParams { email, password }: LoginParams) -> Result<bool> {
        let User {
            id: _,
            email: _,
            password: password_hash,
        } = self
            .repository
            .find_by_email(email)
            .await
            .map_err(|_| Error::FailToLogin)?;

        let password_hash_bytes =
            hex_literal(password_hash.as_str()).ok_or(Error::IncorrectStoredHashFormat)?;

        let result = verify_password(&password.as_bytes(), &password_hash_bytes);

        Ok(result)
    }

    async fn register(&self, RegisterParams { email, password }: RegisterParams) -> Result<i64> {
        // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
        let start = Instant::now();

        debug!("before_hash");

        // https://ryhl.io/blog/async-what-is-blocking/
        let password_encoded = tokio::task::spawn_blocking(move || {
            // This is running on a thread where blocking is fine.
            let start = Instant::now();

            let salt_bytes = generate_salt();
            let hashed_bytes = hash_password(password.as_bytes(), &salt_bytes);

            let after_hash = start.elapsed();

            debug!("after_hash: {after_hash:?}");

            hex(&hashed_bytes)
            // hashed_bytes
        })
        .await
        .unwrap();

        let id = self
            .repository
            .create(CreateParams {
                email,
                password: password_encoded,
            })
            .await;

        let after_create = start.elapsed();

        debug!("after_create: {after_create:?}");

        id
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::{
        error::Result,
        repository::{CreateParams, UserRepository},
    };

    use super::*;

    #[derive(Clone)]
    struct TestUserRepository;
    impl UserRepository for TestUserRepository {
        async fn create(&self, _: CreateParams) -> Result<i64> {
            Ok(42)
        }

        async fn find_by_email<P: AsRef<str> + Sync + Send>(&self, _: P) -> Result<User> {
            Ok(User {
                id: 123,
                email: "aaa@bbb.ccc".to_string(),
                password: "5e3d92cd56f042dfb54620c897cea9160fa46507219333cbc73611a100ea68ed7091885994db88afda2d522fdedca423"
                    .to_string(), //  password:"test"
                                  // "5ee33dd9922ccdd5566ff004422ddffbc2a30dbb7ea074b7c22d4ee60db0b9e3780cd8c514d6fcd7ddef857cf037e23a".to_string(), //  password:"test"
            })
        }
    }

    #[ignore]
    #[tokio::test]
    async fn test_user_service() {
        let repository = TestUserRepository {};
        let serv = UserService::new(repository);
        let password = "test".to_string();
        let register_params = RegisterParams {
            email: "aaa@bbb.ccc".to_string(),
            password: password.clone(),
        };

        let id = serv.register(register_params).await.unwrap();

        assert_eq!(id, 42);
    }

    #[tokio::test]
    async fn test_user_login() {
        let repository = TestUserRepository {};
        let serv = UserService::new(repository);
        let password = "test".to_string();
        let register_params = LoginParams {
            email: "aaa@bbb.ccc".to_string(),
            password: password.clone(),
        };

        let result = serv.login(register_params).await.unwrap();

        assert!(result);
    }
}
