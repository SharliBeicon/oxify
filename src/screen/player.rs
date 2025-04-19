use data::{
    environment::{self, WEBSITE_URL},
    font,
    messages::WelcomeMessage,
};
use iced::{
    alignment,
    widget::{column, container, text},
    Element, Length,
};

#[derive(Debug, Default, Clone)]
pub struct Player;

pub enum PlayerEvent {
    LoginAttempt,
    ReloadConfigAttempt,
}

impl Player {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, message: WelcomeMessage) -> Option<PlayerEvent> {
        use WelcomeMessage as WM;

        match message {
            WM::Login => Some(PlayerEvent::LoginAttempt),
            WM::ReloadConfig => Some(PlayerEvent::ReloadConfigAttempt),
            WM::OpenConfigDir => {
                let _ = open::that_detached(environment::config_dir());

                None
            }
            WM::OpenWebsite => {
                let _ = open::that_detached(WEBSITE_URL);

                None
            }
        }
    }

    pub fn view(&self) -> Element<WelcomeMessage> {
        let content = column![]
            .spacing(1)
            .push(text("LOGGEDDDDDD!").font(font::MONO_BOLD.clone()));

        container(content)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
