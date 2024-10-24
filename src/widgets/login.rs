use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Padding, Paragraph, Widget,
    },
};

use crate::OxifyEvent;

use super::{centered_height, CustomWidget};

#[derive(Debug, Default, Clone)]
pub struct AwaitLogin;

impl CustomWidget for AwaitLogin {
    fn handle_key_event(&self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match key_event.code {
            KeyCode::Char('q') => Some(OxifyEvent::Exit),
            _ => None,
        }
    }
}
impl Widget for AwaitLogin {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = Text::from(" Please, follow the instructions on the browser. ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));

        let block = Block::bordered()
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
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
