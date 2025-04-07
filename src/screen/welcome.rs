use data::{
    config::get_config_mut,
    environment::{self, WEBSITE_URL},
    messages::WelcomeMessage,
};
use iced::{
    alignment,
    widget::{button, column, container, text},
    Element, Length,
};

#[derive(Debug, Default, Clone)]
pub struct Welcome;

pub enum WelcomeEvent {
    LoginAttempt,
}

impl Welcome {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, message: WelcomeMessage) -> Option<WelcomeEvent> {
        match message {
            WelcomeMessage::Login => Some(WelcomeEvent::LoginAttempt),
            WelcomeMessage::ReloadConfig => {
                get_config_mut().reload();

                None
            }

            WelcomeMessage::OpenConfigDir => {
                let _ = open::that_detached(environment::config_dir());

                None
            }
            WelcomeMessage::OpenWebsite => {
                let _ = open::that_detached(WEBSITE_URL);

                None
            }
        }
    }

    pub fn view(&self) -> Element<WelcomeMessage> {
        let config_button = button(
            container(text("Login"))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Shrink),
        )
        .padding([5, 20])
        .width(Length::Shrink)
        .on_press(WelcomeMessage::Login);

        let reload_config = button(
            container(text("ReloadConfig"))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Shrink),
        )
        .padding([5, 20])
        .width(Length::Shrink)
        .on_press(WelcomeMessage::ReloadConfig);

        container(
            column![config_button, reload_config]
                .spacing(20)
                .align_x(alignment::Horizontal::Center),
        )
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
