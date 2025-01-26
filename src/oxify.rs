use std::fmt::Display;

use iced::{
    widget::{button, column, text, Column},
    window, Task,
};
use librespot::oauth::OAuthToken;

use crate::auth;

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
            OAuthError::Error(err) => write!(f, "{}", err.to_string()),
            OAuthError::Undefined => write!(f, "Auth token not defined yet"),
        }
    }
}

pub struct Oxify {
    pub oauth_token: Result<OAuthToken, OAuthError>,
}

impl Default for Oxify {
    fn default() -> Self {
        Self {
            oauth_token: Err(OAuthError::Undefined),
        }
    }
}

impl Oxify {
    pub fn new() -> (Self, Task<Message>) {
        let (main_window, open_main_window) = window::open(window::Settings {
            size: iced::Size::new(400.0, 400.0),
            position: window::Position::Default,
            min_size: Some(iced::Size::new(100.0, 100.0)),
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
                Task::none()
            }
        }
    }

    pub fn view(&self, id: window::Id) -> Column<Message> {
        let token = self.oauth_token.as_ref().map_or_else(
            |err| String::from(err.to_string()),
            |t| t.access_token.clone(),
        );

        column![button("login").on_press(Message::Login), text(token)]
    }
}
