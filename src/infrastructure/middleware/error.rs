use crate::domain::error::Error as DomainError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum AppError {
    LOGIN_FAIL(StatusCode),
    NO_AUTH(StatusCode),
    NOT_FOUND(StatusCode),
    INVALID_PARAMS(StatusCode),
    SERVICE_ERROR(StatusCode),
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::LoginFail => Self::LOGIN_FAIL(StatusCode::FORBIDDEN),
            DomainError::AuthIsNotProvided
            | DomainError::AuthCookieIsEmpty
            | DomainError::AuthCookieWrongFormat => Self::NO_AUTH(StatusCode::FORBIDDEN),
            DomainError::EntityNotFound { .. } => Self::NOT_FOUND(StatusCode::NOT_FOUND),
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::LOGIN_FAIL(status)
            | AppError::NO_AUTH(status)
            | AppError::NOT_FOUND(status)
            | AppError::INVALID_PARAMS(status)
            | AppError::SERVICE_ERROR(status) => status,
        };

        (status, self.as_ref().to_owned()).into_response()
    }
}
