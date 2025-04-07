use crate::log::Record;
use std::fmt::Display;

pub use librespot::oauth::OAuthToken;

#[derive(Debug, Clone)]
pub enum Message {
    Login,
    ReloadConfig,
    Token(Result<OAuthToken, OAuthError>),
    Logging(Vec<Record>),
}

#[derive(Clone, Debug)]
pub enum OAuthError {
    Error(String),
    Undefined,
}

impl Display for OAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthError::Error(err) => write!(f, "{}", err),
            OAuthError::Undefined => write!(f, "Auth token not defined yet"),
        }
    }
}
