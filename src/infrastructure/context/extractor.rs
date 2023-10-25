use std::pin::Pin;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use futures::Future;

use crate::domain::user::error::UserError;
use crate::infrastructure::middleware::error::AppError;

use super::ctx::UserCtx;

impl<S: Send + Sync> FromRequestParts<S> for UserCtx {
    type Rejection = AppError;

    fn from_request_parts<'a, 'b, 'at>(
        parts: &'a mut Parts,
        _state: &'b S,
    ) -> Pin<Box<(dyn Future<Output = Result<UserCtx, Self::Rejection>> + Send + 'at)>>
    where
        'a: 'at,
        'b: 'at,
        Self: 'at,
    {
        Box::pin(async {
            let maybe_ctx = parts.extensions.get::<Result<UserCtx, UserError>>();

            let ctx: &Result<UserCtx, UserError> = maybe_ctx.ok_or(UserError::AuthIsNotProvided)?;

            ctx.clone().map_err(|err| err.into())
        })
    }
}
