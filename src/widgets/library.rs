use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::Stylize,
    style::{Color, Style},
    text::Line,
    widgets::{block::Title, Block, Borders, Widget},
};

use crate::{Focus, OxifyEvent};

use super::{CustomWidget, InputMode};

#[derive(Debug, Default, Clone)]
pub struct Library {
    pub input_mode: InputMode,
}

impl CustomWidget for Library {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') => Some(OxifyEvent::Exit),
                KeyCode::Char('2') => {
                    self.input_mode = InputMode::Focus;
                    Some(OxifyEvent::Focus(Focus::Library))
                }
                _ => None,
            },
            InputMode::Focus => {
                match key_event.code {
                    //KeyCode::Enter => self.submit_message(),
                    KeyCode::Esc => {
                        return Some(OxifyEvent::Focus(Focus::None));
                    }
                    _ => {}
                }
                None
            }
        }
    }
}

impl Widget for Library {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style: Style;
        let title: Title;
        let mut block = Block::bordered();

        if self.input_mode == InputMode::Normal {
            title = Title::from(Line::from(vec![" [2] ".blue().bold(), "Library ".bold()]));
            style = Style::default();
            block = block.borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);
        } else {
            title = Title::from(Line::from(vec![
                " [2] ".light_red().bold(),
                "Library ".bold(),
            ]));
            style = Style::default().fg(Color::Yellow);
        }

        block
            .title(title.alignment(Alignment::Left))
            .style(style)
            .render(area, buf);
    }
}
