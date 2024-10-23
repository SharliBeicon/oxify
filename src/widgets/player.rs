use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{block::Title, Block, Padding, Paragraph, Widget},
};
use symbols::border;

use crate::OxifyEvent;

use super::{centered_height, CustomWidget};

#[derive(Debug, Default, Clone)]
pub struct Player;

impl CustomWidget for Player {
    fn handle_key_event(&self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match key_event.code {
            KeyCode::Char('q') => Some(OxifyEvent::Exit),
            _ => None,
        }
    }

    fn layout(&self, frame: &Frame) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area())
    }
}
impl Widget for Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Player ".bold());
        let content = Text::from(" Player ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .padding(Padding::top(centered_height(
                content.height() as u16,
                &area,
            )))
            .border_set(border::THICK);

        Paragraph::new(content)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
