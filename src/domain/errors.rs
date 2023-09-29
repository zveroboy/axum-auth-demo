use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use strum_macros::AsRefStr;
use tracing::info;

// region: -- Error

#[derive(Clone, Debug)]
pub enum Error {
    LoginFail,

    AuthIsNotProvided,
    AuthCookieIsEmpty,
    AuthCookieWrongFormat,

    EntityNotFound { id: String },
}

impl Error {
    pub fn client_status_code_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::AuthIsNotProvided | Self::AuthCookieIsEmpty | Self::AuthCookieWrongFormat => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            Self::EntityNotFound { .. } => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
        // match self {
        //     Self::LoginFail => write!(f, "LoginFail"),
        // }
    }
}

// impl From<std::env::VarError> for Error  {
//     fn from(value: std::env::VarError) -> Self {
//         Self::ConfigMissing("ttta")
//     }
// }

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        info!("{:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

// endregion: -- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(AsRefStr, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    NOT_FOUND,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
