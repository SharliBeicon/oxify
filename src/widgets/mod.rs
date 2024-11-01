use ratatui::prelude::Rect;
pub mod await_login;
pub mod landing;
pub mod library;
pub mod main_window;
pub mod player;
pub mod popup;
pub mod search;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
}

fn centered_height(element_height: u16, area: &Rect) -> u16 {
    (area.height / 2).saturating_sub((element_height + 1) / 2)
}
