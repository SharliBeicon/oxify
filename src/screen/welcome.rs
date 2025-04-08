use data::{
    config::get_config_mut,
    environment::{self, WEBSITE_URL},
    font,
    messages::WelcomeMessage,
};
use iced::{
    alignment,
    widget::{button, column, container, image, row, text, vertical_space},
    Element, Length,
};

use crate::ui;

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
        let config_dir = String::from(environment::config_dir().to_string_lossy());

        let config_button = button(
            container(text(config_dir))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Shrink),
        )
        .padding([5, 20])
        .width(Length::Shrink)
        .style(|theme, status| ui::button::secondary(theme, status, false))
        .on_press(WelcomeMessage::OpenConfigDir);

        let documentation_button = button(
            container(text("Open Website"))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .padding(5)
        .width(Length::Fill)
        .style(|theme, status| ui::button::secondary(theme, status, false))
        .on_press(WelcomeMessage::OpenWebsite);

        let login_button = button(
            container(text("Login to Spotify").font(font::MONO_BOLD.clone()))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .padding(5)
        .width(Length::Fill)
        .style(|theme, status| ui::button::secondary(theme, status, false))
        .on_press(WelcomeMessage::Login);

        let reload_button = button(
            container(text("Reload Config File"))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .padding(5)
        .width(Length::Fill)
        .style(|theme, status| ui::button::secondary(theme, status, false))
        .on_press(WelcomeMessage::ReloadConfig);

        let logo_bytes = include_bytes!("../../img/oxify-transparent.png").to_vec();
        let content = column![]
            .spacing(1)
            .push(image(image::Handle::from_bytes(logo_bytes)).width(150))
            .push(vertical_space().height(10))
            .push(text("Welcome to Oxify!").font(font::MONO_BOLD.clone()))
            .push(vertical_space().height(10))
            .push(login_button.width(220))
            .push(vertical_space().height(20))
            .push(text("Oxify is configured through a config file."))
            .push(row![
                text("You can find the "),
                text("config.toml").style(ui::text::action),
                text(" file at the following path:"),
            ])
            .push(vertical_space().height(4))
            .push(config_button)
            .push(vertical_space().height(2))
            .push(reload_button.width(220))
            .push(vertical_space().height(10))
            .push(text(
                "All available configuration options are at our website:",
            ))
            .push(vertical_space().height(4))
            .push(documentation_button.width(220))
            .align_x(iced::Alignment::Center);

        container(content)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
