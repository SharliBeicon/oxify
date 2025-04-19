use crate::screen::{Player, Screen, Welcome, WelcomeEvent};
use data::{
    log::Record,
    messages::{Message, OxifyMessage},
    spotify::Setup,
    Config,
};
use iced::{
    widget::container,
    window::{self, Id},
    Element, Size, Task, Theme,
};
use spotify::Service;
use tokio_stream::wrappers::ReceiverStream;

const MIN_SIZE: Size = Size::new(400.0, 300.0);

#[derive(Clone)]
pub struct Oxify {
    pub id: Id,
    pub service: Option<Service>,
    pub screen: Screen,
    pub config: Config,
    pub setup: Setup,
}

impl Oxify {
    pub fn new(
        log_stream: ReceiverStream<Vec<Record>>,
        config: Config,
        setup: Setup,
    ) -> (Self, Task<Message>) {
        let (main_window, open_main_window) = window::open(window::Settings {
            size: config.appaerance.window_size.into(),
            position: window::Position::Default,
            min_size: Some(MIN_SIZE),
            exit_on_close_request: true,
            ..Default::default()
        });

        let service = Service::load(&setup);

        let (oxify, command) = Oxify::load_from_state(main_window, config, service, setup);
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
                OxifyMessage::Token(Some(access_token)) => {
                    let config = self.config.clone();
                    Task::future(async move {
                        Setup::reload(config.clone(), access_token.access_token).await
                    })
                }
                OxifyMessage::ConfigReloaded(config) => {
                    self.config = config;
                    Task::none()
                }
                OxifyMessage::Setup(Some(setup)) => {
                    let service = Service::load(&setup);

                    let (oxify, _) =
                        Oxify::load_from_state(self.id, self.config.clone(), service, setup);

                    *self = oxify;

                    Task::none()
                }
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
            Screen::Player(player) => player.view().map(Message::WelcomeMessage),
        };

        container(content).into()
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        self.config.appaerance.get_theme()
    }

    pub fn load_from_state(
        id: window::Id,
        config: Config,
        service: Option<Service>,
        setup: Setup,
    ) -> (Oxify, Task<Message>) {
        let screen = service.as_ref().map_or_else(
            || Screen::Welcome(Welcome::new()),
            |_| Screen::Player(Player::new()),
        );

        let oxify = Self {
            id,
            service,
            screen,
            config,
            setup,
        };

        (oxify, Task::none())
    }
}
