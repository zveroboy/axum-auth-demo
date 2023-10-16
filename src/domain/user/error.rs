#[derive(Clone, Debug)]
pub enum Error {
    FailedToBuildPasswordHash,
    FailToLogin,

    AuthIsNotProvided,
    AuthCookieIsEmpty,
    AuthCookieWrongFormat,

    EntityNotFound { id: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
