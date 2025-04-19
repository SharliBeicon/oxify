use crate::{config::Config, log::Record, spotify::Setup};
use librespot::oauth::OAuthToken;

#[derive(Debug, Clone)]
pub enum OxifyMessage {
    Logging(Vec<Record>),
    Token(Option<OAuthToken>),
    ConfigReloaded(Config),
    Setup(Option<Setup>),
}

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    Login,
    OpenConfigDir,
    ReloadConfig,
    OpenWebsite,
}

#[derive(Debug, Clone)]
pub enum Message {
    OxifyMessage(OxifyMessage),
    WelcomeMessage(WelcomeMessage),
}
