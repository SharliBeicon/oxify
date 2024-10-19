use crate::OxifyEvent;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, widgets::Widget};

mod landing;
pub use landing::Landing;

mod popup;
pub use popup::{Popup, PopupKind};

pub mod login;

pub trait CustomWidget: Widget + Clone {
    fn handle_key_event(&self, key_event: KeyEvent) -> Option<OxifyEvent>;
}

fn centered_height(element_height: u16, area: &Rect) -> u16 {
    (area.height / 2) - ((element_height + 1) / 2)
}
