#[derive(Clone, Debug)]
pub enum UserError {
    FailedToBuildPasswordHash,
    IncorrectStoredHashFormat,
    FailToLogin,

    AuthIsNotProvided,
    AuthCookieIsEmpty,
    AuthCookieWrongFormat,

    EntityNotFound { id: String },
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for UserError {}

pub type UserResult<T> = core::result::Result<T, UserError>;
