use std::convert::Infallible;

use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use super::AUTH_TOKEN;
// use crate::domain::error::Error;

use crate::domain::user::error::UserError;
use crate::infrastructure::auth::jwt::Jwt;
use crate::infrastructure::context::ctx::UserCtx;
use crate::infrastructure::state::AppState;

pub async fn auth_resolver<B>(
    State(state): State<AppState>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, Infallible> {
    let auth_cookie_result = cookies.get(AUTH_TOKEN).ok_or(UserError::AuthCookieIsEmpty);

    let result_ctx = auth_cookie_result
        .and_then(|auth_cookie| {
            Jwt::<'_, UserCtx>::try_decode(&state.config.jwt_secret, auth_cookie.value())
                .map_err(|_| UserError::AuthCookieWrongFormat)
        })
        .map(|jwt| jwt.claims);

    debug!("result_ctx: {:?}", result_ctx);

    if result_ctx.is_err() {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    let response = next.run(req).await;

    Ok(response)
}
