use crate::{
    context::config::Config,
    data::{
        log::Record,
        messages::{Message, OxifyMessage},
    },
    screen::{Screen, Welcome, WelcomeEvent},
};
use iced::{
    widget::container,
    window::{self, Id},
    Element, Size, Task, Theme,
};
use tokio_stream::wrappers::ReceiverStream;

const MIN_SIZE: Size = Size::new(400.0, 300.0);

#[derive(Clone)]
pub struct Oxify {
    pub id: Id,
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

        let oxify = Self {
            id: main_window,
            screen: Screen::Welcome(Welcome::new()),
            config,
        };

        let commands = vec![
            open_main_window.then(|_| Task::none()),
            Task::stream(log_stream).map(|ls| Message::OxifyMessage(OxifyMessage::Logging(ls))),
        ];

        (oxify, Task::batch(commands))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OxifyMessage(oxify_message) => match oxify_message {
                OxifyMessage::Logging(records) => Task::none(),
                OxifyMessage::ConfigReloaded(config) => {
                    self.config = config;
                    Task::none()
                }
            },
            Message::WelcomeMessage(welcome_message) => {
                let Screen::Welcome(welcome) = &mut self.screen else {
                    return Task::none();
                };

                match welcome.update(welcome_message) {
                    Some(event) => match event {
                        WelcomeEvent::LoginAttempt => todo!(),
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
        };

        container(content).into()
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        self.config.appaerance.get_theme()
    }
}
