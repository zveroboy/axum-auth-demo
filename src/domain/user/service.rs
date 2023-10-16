use futures::Future;
use password_auth::{generate_hash, verify_password};

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
    fn login(&mut self, params: LoginParams) -> impl Future<Output = Result<bool>> + Send;

    fn register(&self, params: RegisterParams) -> impl Future<Output = Result<i64>> + Send;

    // async fn confirm(&mut self, id: u32) -> Result<()>;
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
    async fn login(&mut self, LoginParams { email, password }: LoginParams) -> Result<bool> {
        let User {
            id: _,
            email: _,
            password: password_hash,
        } = self
            .repository
            .find_by_email(email)
            .await
            .map_err(|_| Error::FailToLogin)?;

        let result = verify_password(&password, &password_hash);

        println!(
            "password: {:?} {:?} {:?}",
            &password, &password_hash, &result
        );

        Ok(result.is_ok())
    }

    async fn register(&self, RegisterParams { email, password }: RegisterParams) -> Result<i64> {
        let password_hash = generate_hash(&password);

        self.repository
            .create(CreateParams {
                email,
                password: password_hash,
            })
            .await
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
                password: "hA$heD".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_user_service() {
        let repository = TestUserRepository {};
        let serv = UserService::new(repository);
        let password = "secret".to_string();
        let register_params = RegisterParams {
            email: "aaa@bbb.ccc".to_string(),
            password: password.clone(),
        };

        let id = serv.register(register_params).await.unwrap();

        assert_eq!(id, 42);

        let hash_example = "$argon2id$v=19$m=19456,t=2,p=1$2maSmQC5KfVAD0AO3lrWog$ULXS5tWR/8fQIXp0ZHe0kFnjcoif1zunOgkf02eJtos";

        assert!(verify_password(password, hash_example).is_ok());
    }
}
