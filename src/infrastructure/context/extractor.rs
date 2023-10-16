use std::pin::Pin;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use futures::Future;

use crate::domain::error::Error;
use crate::infrastructure::middleware::error::AppError;

use super::ctx::UserCtx;

impl<S: Send + Sync> FromRequestParts<S> for UserCtx {
    type Rejection = AppError;

    fn from_request_parts<'a, 'b, 'at>(
        parts: &'a mut Parts,
        _state: &'b S,
    ) -> Pin<Box<(dyn Future<Output = Result<UserCtx, AppError>> + Send + 'at)>>
    where
        'a: 'at,
        'b: 'at,
        Self: 'at,
    {
        Box::pin(async {
            let ctx = parts
                .extensions
                .get::<Result<UserCtx, AppError>>()
                .ok_or(Error::AuthIsNotProvided)?;

            ctx.clone()
        })
    }
}
