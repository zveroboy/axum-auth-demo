use std::time::Instant;

use crate::domain::user::error::UserError;
use crate::domain::user::service::{LoginParams, RegisterParams, UserCommands};
use crate::infrastructure::context::ctx::UserCtx;
use crate::infrastructure::middleware::error::AppError;
use crate::infrastructure::middleware::AUTH_TOKEN;
use crate::infrastructure::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use super::dto::RegisterDto;
use super::jwt::Jwt;
use super::service::BaseUserAppState;

async fn handle_register(
    BaseUserAppState { user_service }: BaseUserAppState,
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

async fn handle_login(
    state: State<AppState>,
    BaseUserAppState { user_service }: BaseUserAppState,
    cookies: Cookies,
    dto: Json<super::dto::LoginDto>,
) -> Result<StatusCode, AppError> {
    let start = Instant::now();

    let user_id = user_service
        .login(LoginParams {
            email: dto.email.clone(),
            password: dto.password.clone(),
        })
        .await?;

    debug!("after login {:?}", start.elapsed());

    let jwt = Jwt::<'_, UserCtx>::new(
        &state.config.jwt_secret,
        Default::default(),
        UserCtx::new(user_id as u32),
    );

    debug!("after jwt {:?}", start.elapsed());

    let token = jwt.encode().map_err(|_| UserError::FailToLogin)?;
    debug!("after jwt encode {:?}", start.elapsed());

    cookies.add(Cookie::build(AUTH_TOKEN, token).path("/").finish());
    debug!("after cookie add {:?}", start.elapsed());

    Ok(StatusCode::OK)
}

async fn handle_logout(cookies: Cookies) -> Result<StatusCode, AppError> {
    cookies.remove(Cookie::named(AUTH_TOKEN));
    Ok(StatusCode::OK)
}

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handle_register))
        .route("/login", post(handle_login))
        .route("/logout", post(handle_logout))
}
