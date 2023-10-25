use std::convert::Infallible;
use std::fmt::Debug;
use std::pin::Pin;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use futures::Future;
use sqlx::PgPool;
use tracing::debug;

use crate::domain::user::entity::User;
use crate::domain::user::error::UserResult;
use crate::domain::user::repository::{CreateParams, UserRepository};
use crate::domain::user::service::UserService;
use crate::infrastructure::store::Db;

#[derive(Clone)]
pub struct SqlxUserRepository {
    db: Db,
}

impl SqlxUserRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl UserRepository for SqlxUserRepository {
    async fn create(&self, CreateParams { email, password }: CreateParams) -> UserResult<i64> {
        debug!("create: {email}, {password}");
        let (id,) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO \"user\"(email, password) VALUES ($1, $2) RETURNING id",
        )
        .bind(email)
        .bind(password)
        .fetch_one(&self.db)
        .await
        .unwrap(); // TODO: handle sqlx error

        println!("created: {id}");
        Ok(id)
    }

    async fn find_by_email<P: AsRef<str> + Sync + Send + Debug>(
        &self,
        email: P,
    ) -> UserResult<User> {
        println!("find_by_email: {:?}", &email);
        let user: User = sqlx::query_as("SELECT * FROM \"user\" WHERE email=$1 LIMIT 1")
            .bind(email.as_ref())
            .fetch_one(&self.db)
            .await
            .unwrap(); // TODO: handle sqlx error, e.g. Err::RowNotFound

        Ok(user)
    }
}

pub type RestUserService = UserService<SqlxUserRepository>;

#[derive(Clone, FromRef)]
pub struct BaseUserAppState {
    pub user_service: RestUserService,
}

impl<S: Send + Sync> FromRequestParts<S> for BaseUserAppState
where
    PgPool: FromRef<S>,
{
    type Rejection = Infallible;

    fn from_request_parts<'a, 'b, 'at>(
        _parts: &'a mut Parts,
        state: &'b S,
    ) -> Pin<Box<(dyn Future<Output = Result<BaseUserAppState, Self::Rejection>> + Send + 'at)>>
    where
        'a: 'at,
        'b: 'at,
        Self: 'at,
    {
        Box::pin(async {
            let pool = PgPool::from_ref(state);
            let user_service = UserService::new(SqlxUserRepository::new(pool.clone()));

            Ok(BaseUserAppState { user_service })
        })
    }
}
