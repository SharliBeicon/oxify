use crate::OxifyEvent;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, widgets::Widget};

// >>> MainWindow widgets >>>
mod library;
pub mod main_window;
mod player;
mod search;
// <<< MainWindow widgets <<<

mod landing;
pub use landing::Landing;

mod popup;
pub use popup::{Popup, PopupKind};

pub mod login;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Focus,
}

pub trait CustomWidget: Widget + Clone {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<OxifyEvent>;
}

fn centered_height(element_height: u16, area: &Rect) -> u16 {
    ((area.height as i32 / 2) - ((element_height as i32 + 1) / 2))
        .try_into()
        .unwrap_or(0)
}
