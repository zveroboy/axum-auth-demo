use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::domain::error::Error;
use crate::infrastructure::middleware::error::AppError;

use super::ctx::UserCtx;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for UserCtx {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, AppError> {
        let ctx = parts
            .extensions
            .get::<Result<UserCtx, AppError>>()
            .ok_or(Error::AuthIsNotProvided)?;
        ctx.clone()
    }
}
