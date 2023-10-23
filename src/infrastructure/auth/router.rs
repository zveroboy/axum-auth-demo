use crate::domain::user::error::UserError;
use crate::domain::user::service::{LoginParams, RegisterParams, UserCommands};
use crate::infrastructure::middleware::error::AppError;
use crate::infrastructure::middleware::AUTH_TOKEN;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use tower_cookies::{Cookie, Cookies};

use super::dto::RegisterDto;

async fn handle_register<Serv: UserCommands>(
    State(user_service): State<Serv>,
    Json(dto): Json<RegisterDto>,
) -> Result<Json<i64>, AppError> {
    let res = user_service
        .register(RegisterParams {
            email: dto.email,
            password: dto.password,
        })
        .await?;

    Ok(res.into())
}

async fn handle_login<Serv: UserCommands>(
    State(user_service): State<Serv>,
    cookies: Cookies,
    dto: Json<super::dto::LoginDto>,
) -> Result<StatusCode, AppError> {
    let matched = user_service
        .login(LoginParams {
            email: dto.email.clone(),
            password: dto.password.clone(),
        })
        .await?;

    if !matched {
        return { Err(UserError::FailToLogin) }?;
    }

    // FIXME:
    let value = format!("user:321.123.456");

    cookies.add(Cookie::build(AUTH_TOKEN, value).path("/").finish());

    Ok(StatusCode::OK)
}

async fn handle_logout(cookies: Cookies) -> Result<StatusCode, AppError> {
    cookies.remove(Cookie::named(AUTH_TOKEN));
    Ok(StatusCode::OK)
}

pub fn auth_router<Serv>() -> Router<Serv>
where
    Serv: UserCommands + 'static + Clone + Sync + Send,
{
    Router::new()
        .route("/register", post(handle_register::<Serv>))
        .route("/login", post(handle_login::<Serv>))
        .route("/logout", post(handle_logout))
}
