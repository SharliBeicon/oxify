use crate::screen::{Screen, Welcome, WelcomeEvent};
use data::{
    config::CONFIG as config,
    log::Record,
    messages::{Message, OxifyMessage},
};
use iced::{widget::container, window, Element, Size, Task, Theme};
use spotify::auth::OAuthToken;
use tokio_stream::wrappers::ReceiverStream;

const MIN_SIZE: Size = Size::new(400.0, 300.0);

pub struct Oxify {
    pub oauth_token: Option<OAuthToken>,
    pub screen: Screen,
}

impl Oxify {
    pub fn new(log_stream: ReceiverStream<Vec<Record>>) -> (Self, Task<Message>) {
        let (main_window, open_main_window) = window::open(window::Settings {
            size: config.blocking_read().window_size.into(),
            position: window::Position::Default,
            min_size: Some(MIN_SIZE),
            exit_on_close_request: true,
            ..Default::default()
        });

        let (oxify, command) = Oxify::load_from_state(main_window);
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
                OxifyMessage::Logging(_) => Task::none(),
                OxifyMessage::Token(_) => todo!(),
                _ => Task::none(),
            },
            Message::WelcomeMessage(welcome_message) => {
                let Screen::Welcome(welcome) = &mut self.screen else {
                    return Task::none();
                };

                match welcome.update(welcome_message) {
                    Some(event) => match event {
                        WelcomeEvent::LoginAttempt => Task::future(spotify::auth::login()),
                        WelcomeEvent::ReloadConfigAttempt => {
                            Task::future(async { config.write().await.reload().await })
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
        config.blocking_read().get_theme()
    }

    pub fn load_from_state(_: window::Id) -> (Oxify, Task<Message>) {
        let oxify = Self {
            oauth_token: None,
            screen: Screen::Welcome(Welcome::new()),
        };
        (oxify, Task::none())
    }
}
