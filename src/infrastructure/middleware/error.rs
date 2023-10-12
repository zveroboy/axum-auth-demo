use crate::domain::errors::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL(StatusCode),
    NO_AUTH(StatusCode),
    NOT_FOUND(StatusCode),
    INVALID_PARAMS(StatusCode),
    SERVICE_ERROR(StatusCode),
}

impl From<Error> for ClientError {
    fn from(error: Error) -> Self {
        match error {
            Error::LoginFail => Self::LOGIN_FAIL(StatusCode::FORBIDDEN),
            Error::AuthIsNotProvided | Error::AuthCookieIsEmpty | Error::AuthCookieWrongFormat => {
                Self::NO_AUTH(StatusCode::FORBIDDEN)
            }
            Error::EntityNotFound { .. } => Self::NOT_FOUND(StatusCode::NOT_FOUND),
            _ => Self::SERVICE_ERROR(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

// impl From<Error> for ClientError {
//     fn from(&self) -> (StatusCode, ClientError) {
//         match self {
//             Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
//             Self::AuthIsNotProvided | Self::AuthCookieIsEmpty | Self::AuthCookieWrongFormat => {
//                 (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
//             }
//             Self::EntityNotFound { .. } => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),
//             _ => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 ClientError::SERVICE_ERROR,
//             ),
//         }
//     }
// }

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        let status = match self {
            ClientError::LOGIN_FAIL(status) => status,
            ClientError::NO_AUTH(status) => status,
            ClientError::NOT_FOUND(status) => status,
            ClientError::INVALID_PARAMS(status) => status,
            ClientError::SERVICE_ERROR(status) => status,
        };

        (status, self.as_ref().to_owned()).into_response()
    }
}
