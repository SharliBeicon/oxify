use crate::screen::{Screen, Welcome};
use data::log::Record;
use data::messages::OxifyMessage;
use data::{config::Config, messages::OAuthToken};
use iced::{widget::container, window, Element, Size, Task, Theme};
use tokio_stream::wrappers::ReceiverStream;

const MIN_SIZE: Size = Size::new(400.0, 300.0);

pub struct Oxify {
    pub oauth_token: Option<OAuthToken>,
    pub screen: Screen,
    pub config: Config,
}

impl Oxify {
    pub fn new(
        config: Config,
        log_stream: ReceiverStream<Vec<Record>>,
    ) -> (Self, Task<OxifyMessage>) {
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
            Task::stream(log_stream).map(OxifyMessage::Logging),
        ];

        (oxify, Task::batch(commands))
    }

    pub fn update(&mut self, message: OxifyMessage) -> Task<OxifyMessage> {
        match message {
            OxifyMessage::Login => Task::perform(spotify::auth::login(), |msg: OxifyMessage| msg),
            OxifyMessage::Token(res) => {
                self.oauth_token = res;
                Task::none()
            }
            OxifyMessage::ReloadConfig => {
                self.config.reload();
                Task::none()
            }
            OxifyMessage::Logging(_) => Task::none(),
        }
    }

    pub fn view(&self, _: window::Id) -> Element<OxifyMessage> {
        let content = match &self.screen {
            Screen::Welcome(welcome) => welcome.view(),
        };

        container(content).into()
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        self.config.get_theme()
    }

    pub fn load_from_state(config: Config, _: window::Id) -> (Oxify, Task<OxifyMessage>) {
        let oxify = Self {
            oauth_token: None,
            screen: Screen::Welcome(Welcome::new()),
            config,
        };
        (oxify, Task::none())
    }
}
