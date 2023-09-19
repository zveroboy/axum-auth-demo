use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use tower_cookies::Cookies;

use lazy_regex::regex_captures;
use tracing::{debug, info};

use crate::{
    domain::errors::{Error, Result},
    infrastructure::auth::AUTH_TOKEN,
};

use super::ctx::UserCtx;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for UserCtx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let token_cookie = cookies.get(AUTH_TOKEN).ok_or(Error::AuthIsNotProvided)?;

        info!(token = token_cookie.value(), "UserCtx Extractor");

        let (user_id, _, _) = parse_auth(token_cookie.value())?;

        Ok(UserCtx::new(user_id))
    }
}

pub fn parse_auth(token: &str) -> Result<(u32, String, String)> {
    debug!(token);

    // user-<user_id>.<expiration>.<signature>
    let (_, user_id, expiration, signature) =
        regex_captures!(r#"^user:(\d+)\.(\d+)\.(\d+)"#, &token)
            .ok_or(Error::AuthCookieWrongFormat)?;

    let user_id = user_id
        .parse::<u32>()
        .map_err(|_| Error::AuthCookieWrongFormat)?;

    Ok((user_id, expiration.to_string(), signature.to_string()))
}
