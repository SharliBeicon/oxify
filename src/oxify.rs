use crate::screen::{Screen, Welcome, WelcomeEvent};
use data::{
    log::Record,
    messages::{Message, OxifyMessage},
    Config,
};
use iced::{widget::container, window, Element, Size, Task, Theme};
use spotify::auth::OAuthToken;
use tokio_stream::wrappers::ReceiverStream;

const MIN_SIZE: Size = Size::new(400.0, 300.0);

pub struct Oxify {
    pub oauth_token: Option<OAuthToken>,
    pub screen: Screen,
    pub config: Config,
}

impl Oxify {
    pub fn new(log_stream: ReceiverStream<Vec<Record>>, config: Config) -> (Self, Task<Message>) {
        let (main_window, open_main_window) = window::open(window::Settings {
            size: config.appaerance.window_size.into(),
            position: window::Position::Default,
            min_size: Some(MIN_SIZE),
            exit_on_close_request: true,
            ..Default::default()
        });

        let (oxify, command) = Oxify::load_from_state(main_window, config);
        let commands = vec![
            open_main_window.then(|_| Task::none()),
            command,
            Task::stream(log_stream).map(|ls| Message::OxifyMessage(OxifyMessage::Logging(ls))),
        ];

        (oxify, Task::batch(commands))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OxifyMessage(oxify_message) => match oxify_message {
                OxifyMessage::Logging(_) => ().into(),
                OxifyMessage::Token(_) => ().into(),
                OxifyMessage::ConfigReloaded(config) => (self.config = config).into(),
            },
            Message::WelcomeMessage(welcome_message) => {
                let Screen::Welcome(welcome) = &mut self.screen else {
                    return Task::none();
                };

                match welcome.update(welcome_message) {
                    Some(event) => match event {
                        WelcomeEvent::LoginAttempt => Task::future(spotify::auth::login()),
                        WelcomeEvent::ReloadConfigAttempt => {
                            let config = self.config.clone();
                            Task::future(async move { config.reload().await })
                        }
                    },
                    None => Task::none(),
                }
            }
        }
    }

    pub fn view(&self, _: window::Id) -> Element<Message> {
        let content = match &self.screen {
            Screen::Welcome(welcome) => welcome.view().map(Message::WelcomeMessage),
            Screen::Oxify => todo!(),
        };

        container(content).into()
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        self.config.appaerance.get_theme()
    }

    pub fn load_from_state(_: window::Id, config: Config) -> (Oxify, Task<Message>) {
        let oxify = Self {
            oauth_token: None,
            screen: Screen::Welcome(Welcome::new()),
            config,
        };
        (oxify, Task::none())
    }
}
