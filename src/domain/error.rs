use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::info;

// region: -- Error

#[derive(Clone, Debug)]
pub enum Error {
    AuthIsNotProvided,
    AuthCookieIsEmpty,
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

// impl From<std::env::VarError> for Error  {
//     fn from(value: std::env::VarError) -> Self {
//         Self::ConfigMissing("ttta")
//     }
// }

impl std::error::Error for Error {}

// impl IntoResponse for Error {
//     fn into_response(self) -> Response {
//         info!("{:<12} - {self:?}", "INTO_RES");

//         let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

//         response.extensions_mut().insert(self);

//         response
//     }
// }

// endregion: -- Error

pub type Result<T> = core::result::Result<T, Error>;
