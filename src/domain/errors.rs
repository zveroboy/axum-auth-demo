use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::info;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    AuthIsNotProvided,
    AuthCookieWrongFormat,

    EntityNotFound { id: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
        // match self {
        //     Self::LoginFail => write!(f, "LoginFail"),
        // }
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        info!("{:<12} - {self:?}", "INTO_RES");
        // TODO: add logging
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_ERROR").into_response()
    }
}

pub type Result<T> = core::result::Result<T, Error>;
