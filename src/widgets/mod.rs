use ratatui::{layout::Rect, widgets::Widget};
use std::io;
mod landing;
mod login;
use crate::InternalMessage;
pub use landing::Landing;

pub trait CustomWidget: Widget + Clone + Copy {
    fn handle_events(&mut self) -> io::Result<Option<InternalMessage>>;
}

fn centered_height(element_height: u16, area: &Rect) -> u16 {
    (area.height / 2) - ((element_height + 1) / 2)
}
