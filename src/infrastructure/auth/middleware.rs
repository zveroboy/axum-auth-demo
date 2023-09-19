use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{domain::errors::Result, infrastructure::auth::AUTH_TOKEN};

use super::context::ctx::UserCtx;

pub async fn require_auth<B>(
    // user_ctx: UserCtx,
    user_ctx: Result<UserCtx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!("require_auth");

    user_ctx?;

    // let _auth_cookie = cookies
    //     .get(AUTH_TOKEN)
    //     .map(|c| c.to_string())
    //     .ok_or(Error::AuthIsNotProvided)
    //     .and_then(parse_auth)?;

    Ok(next.run(req).await)
}


// pub async fn auth_resolver<B>(
//     cookies: Cookies,
//     req: Request<B>,
//     next: Next<B>,
// ) -> Result<Response> {
//     let auth_cookie = cookies.get(AUTH_TOKEN);

//     info!(auth_cookie., "auth_resolver");

//     // user_ctx?;

//     // let _auth_cookie = cookies
//     //     .get(AUTH_TOKEN)
//     //     .map(|c| c.to_string())
//     //     .ok_or(Error::AuthIsNotProvided)
//     //     .and_then(parse_auth)?;

//     Ok(next.run(req).await)
// }

