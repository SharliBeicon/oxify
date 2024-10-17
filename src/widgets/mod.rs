use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, widgets::Widget};
mod landing;
pub mod login;
use crate::Event;
pub use landing::Landing;

pub trait CustomWidget: Widget + Clone + Copy {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Event>;
}

fn centered_height(element_height: u16, area: &Rect) -> u16 {
    (area.height / 2) - ((element_height + 1) / 2)
}
