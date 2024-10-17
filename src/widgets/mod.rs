use ratatui::layout::Rect;

pub mod app;
pub mod login;

fn centered_height(element_height: u16, area: &Rect) -> u16 {
    (area.height / 2) - ((element_height + 1) / 2)
}
