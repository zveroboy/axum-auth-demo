use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::{domain::errors::Error, infrastructure::middleware::error::ClientError};

use super::ctx::UserCtx;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for UserCtx {
    type Rejection = ClientError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, ClientError> {
        let ctx = parts
            .extensions
            .get::<Result<UserCtx, ClientError>>()
            .ok_or(Error::AuthIsNotProvided)?;
        ctx.clone()
    }
}
