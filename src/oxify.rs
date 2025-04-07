use crate::{
    auth,
    config::Config,
    logger,
    screen::{Screen, Welcome},
};
use data::log::Record;
use iced::{widget::container, window, Element, Size, Task, Theme};
use librespot::oauth::OAuthToken;
use std::fmt::Display;
use tokio_stream::wrappers::ReceiverStream;

const MIN_SIZE: Size = Size::new(400.0, 300.0);

#[derive(Debug, Clone)]
pub enum Message {
    Login,
    ReloadConfig,
    Token(Result<OAuthToken, OAuthError>),
    Logging(Vec<logger::Record>),
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
    pub config: Config,
}

impl Oxify {
    pub fn new(config: Config, log_stream: ReceiverStream<Vec<Record>>) -> (Self, Task<Message>) {
        let (main_window, open_main_window) = window::open(window::Settings {
            size: config.window_size.into(),
            position: window::Position::Default,
            min_size: Some(MIN_SIZE),
            exit_on_close_request: true,
            ..Default::default()
        });

        let (oxify, command) = Oxify::load_from_state(config, main_window);
        let commands = vec![
            open_main_window.then(|_| Task::none()),
            command,
            Task::stream(log_stream).map(Message::Logging),
        ];

        (oxify, Task::batch(commands))
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
            Message::ReloadConfig => {
                self.config.reload();
                Task::none()
            }
            Message::Logging(_) => Task::none(),
        }
    }

    pub fn view(&self, _: window::Id) -> Element<Message> {
        let content = match &self.screen {
            Screen::Welcome(welcome) => welcome.view(),
        };

        container(content).into()
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        self.config.get_theme()
    }

    pub fn load_from_state(config: Config, _: window::Id) -> (Oxify, Task<Message>) {
        let oxify = Self {
            oauth_token: Err(OAuthError::Undefined),
            screen: Screen::Welcome(Welcome::new()),
            config,
        };
        (oxify, Task::none())
    }
}
