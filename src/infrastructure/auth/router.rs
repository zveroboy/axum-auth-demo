use crate::domain::errors::Error;
use crate::infrastructure::middleware::error::ClientError;
use crate::infrastructure::middleware::AUTH_TOKEN;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use tower_cookies::{Cookie, Cookies};

async fn handle_login(
    cookies: Cookies,
    dto: Json<super::dto::LoginDto>,
) -> Result<StatusCode, ClientError> {
    // TODO: move to domain
    if dto.email != "demo" || dto.password != "test" {
        return { Err(Error::LoginFail) }?;
    }

    // FIXME:
    cookies.add(
        Cookie::build(AUTH_TOKEN, "user:321.123.456")
            .path("/")
            .finish(),
    );

    Ok(StatusCode::OK)
}

pub fn auth_router() -> Router {
    Router::new().route("/login", post(handle_login))
}
