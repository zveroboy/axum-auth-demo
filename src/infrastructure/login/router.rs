use crate::domain::errors::Error;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use tower_cookies::{Cookie, Cookies};

async fn login(cookies: Cookies, dto: Json<super::dto::LoginDto>) -> impl IntoResponse {
    println!("login cookies {:?}", &cookies.list());

    if dto.email != "demo" || dto.password != "test" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(crate::infrastructure::login::AUTH_TOKEN, "user:demo.exp.sign"));

    Ok(StatusCode::OK)
}

pub fn auth_router() -> Router {
    Router::new().route("/login", post(login))
}
