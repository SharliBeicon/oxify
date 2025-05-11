use super::log::Record;
use crate::context::config::Config;

#[derive(Debug, Clone)]
pub enum Message {
    OxifyMessage(OxifyMessage),
    WelcomeMessage(WelcomeMessage),
}

#[derive(Debug, Clone)]
pub enum OxifyMessage {
    Logging(Vec<Record>),
    ConfigReloaded(Config),
}

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    Login,
    OpenConfigDir,
    ReloadConfig,
    OpenWebsite,
}
