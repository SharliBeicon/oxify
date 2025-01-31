use iced::{widget::container, window, Element, Size, Task, Theme};
use librespot::oauth::OAuthToken;
use std::fmt::Display;

use crate::{
    auth,
    config::CONFIG,
    screen::{Screen, Welcome},
};

const MIN_SIZE: Size = Size::new(400.0, 300.0);

#[derive(Debug, Clone)]
pub enum Message {
    Login,
    Token(Result<OAuthToken, OAuthError>),
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

pub struct Oxify {
    pub oauth_token: Result<OAuthToken, OAuthError>,
    pub screen: Screen,
}

impl Default for Oxify {
    fn default() -> Self {
        Self {
            oauth_token: Err(OAuthError::Undefined),
            screen: Screen::Welcome(Welcome::new()),
        }
    }
}

impl Oxify {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open_main_window) = window::open(window::Settings {
            size: CONFIG.window_size.into(),
            position: window::Position::Default,
            min_size: Some(MIN_SIZE),
            exit_on_close_request: true,
            ..Default::default()
        });
        (Self::default(), open_main_window.then(|_| Task::none()))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login => Task::perform(auth::login(), |msg: Message| msg),
            Message::Token(res) => {
                if let Err(err) = &res {
                    log::error!("Cannot get access token: {}", err);
                }
                self.oauth_token = res;
                println!("{:?}", self.oauth_token);
                Task::none()
            }
        }
    }

    pub fn view(&self, _: window::Id) -> Element<Message> {
        let content = match &self.screen {
            Screen::Welcome(welcome) => welcome.view(),
        };

        container(content).into()
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        CONFIG.get_theme()
    }
}
