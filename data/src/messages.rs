use crate::log::Record;

pub use librespot::oauth::OAuthToken;

#[derive(Debug, Clone)]
pub enum OxifyMessage {
    Login,
    ReloadConfig,
    Token(Option<OAuthToken>),
    Logging(Vec<Record>),
}
