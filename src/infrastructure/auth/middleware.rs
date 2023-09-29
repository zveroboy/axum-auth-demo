use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::domain::errors::{Error, Result};
use crate::infrastructure::auth::AUTH_TOKEN;

use super::context::ctx::UserCtx;

pub async fn auth_resolver<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_cookie = cookies.get(AUTH_TOKEN).ok_or(Error::AuthCookieIsEmpty)?;
    let result_ctx = parse_auth(auth_cookie.value()).map(|(user_id, _, _)| UserCtx::new(user_id));

    if result_ctx.is_err() {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

fn parse_auth(token: &str) -> Result<(u32, String, String)> {
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
