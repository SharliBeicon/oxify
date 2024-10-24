use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::OxifyEvent;

use super::CustomWidget;

#[derive(Debug, Default, Clone)]
pub struct Library;

impl CustomWidget for Library {
    fn handle_key_event(&self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match key_event.code {
            KeyCode::Char('q') => Some(OxifyEvent::Exit),
            _ => None,
        }
    }
}

impl Widget for Library {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
