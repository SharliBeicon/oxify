use crate::log::Record;
use librespot::oauth::OAuthToken;

#[derive(Debug, Clone)]
pub enum OxifyMessage {
    Logging(Vec<Record>),
    Token(Option<OAuthToken>),
}

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    Login,
    ReloadConfig,
    OpenConfigDir,
    OpenWebsite,
}

#[derive(Debug, Clone)]
pub enum Message {
    OxifyMessage(OxifyMessage),
    WelcomeMessage(WelcomeMessage),
}
