use std::convert::Infallible;

use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use super::AUTH_TOKEN;
use crate::domain::error::Error;

use crate::infrastructure::context::ctx::UserCtx;

struct Token(u32, String, String);

impl Token {
    fn get_user_id(self: &Self) -> u32 {
        self.0
    }
}

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_ctx_require<B>(
    ctx: Result<UserCtx, Error>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Error> {
    ctx?;

    Ok(next.run(req).await)
}

pub async fn auth_resolver<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, Infallible> {
    let auth_cookie_result = cookies.get(AUTH_TOKEN).ok_or(Error::AuthCookieIsEmpty);

    let result_ctx = auth_cookie_result
        .and_then(|auth_cookie| parse_auth(auth_cookie.value()))
        .map(|token| UserCtx::new(token.get_user_id()));

    if result_ctx.is_err() {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

fn parse_auth<T: AsRef<str>>(token: T) -> Result<Token, Error> {
    let (_, user_id, expiration, signature) =
        regex_captures!(r#"^user:(\d+)\.(\d+)\.(\d+)"#, token.as_ref())
            .ok_or(Error::AuthCookieWrongFormat)?;

    let user_id = user_id
        .parse::<u32>()
        .map_err(|_| Error::AuthCookieWrongFormat)?;

    Ok(Token(
        user_id,
        expiration.to_string(),
        signature.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_parse_cookie() {}
}
