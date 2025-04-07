use data::messages::OxifyMessage;
use iced::{
    alignment,
    widget::{button, column, container, text},
    Element, Length,
};

pub struct Welcome {}

impl Default for Welcome {
    fn default() -> Self {
        Self::new()
    }
}

impl Welcome {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<OxifyMessage> {
        let config_button = button(
            container(text("Login"))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Shrink),
        )
        .padding([5, 20])
        .width(Length::Shrink)
        .on_press(OxifyMessage::Login);

        let reload_config = button(
            container(text("ReloadConfig"))
                .align_x(alignment::Horizontal::Center)
                .width(Length::Shrink),
        )
        .padding([5, 20])
        .width(Length::Shrink)
        .on_press(OxifyMessage::ReloadConfig);

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
