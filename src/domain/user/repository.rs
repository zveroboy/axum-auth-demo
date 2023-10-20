use std::fmt::Debug;

use futures::Future;

use super::entity::User;
use super::error::UserResult;

pub struct CreateParams {
    pub email: String,
    pub password: String,
}

pub trait UserRepository: Sync + Send + Clone {
    fn create(&self, params: CreateParams) -> impl Future<Output = UserResult<i64>> + Send;
    fn find_by_email<P: AsRef<str> + Sync + Send + Debug>(
        &self,
        email: P,
    ) -> impl Future<Output = UserResult<User>> + Send;
}
