use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::Stylize,
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{block::Title, Block, Borders, Widget},
};

use crate::{Focus, OxifyEvent};

use super::{CustomWidget, InputMode};

#[derive(Debug, Default, Clone)]
pub struct Search {
    input: String,
    character_index: usize,
    pub input_mode: InputMode,
}

impl Search {
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        if self.character_index != 0 {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
}

impl CustomWidget for Search {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') => Some(OxifyEvent::Exit),
                KeyCode::Char('1') => {
                    self.input_mode = InputMode::Focus;
                    Some(OxifyEvent::Focus(Focus::Search))
                }
                _ => None,
            },
            InputMode::Focus => {
                match key_event.code {
                    //KeyCode::Enter => self.submit_message(),
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
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

impl Widget for Search {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let search_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.horizontal_down,
            ..symbols::border::PLAIN
        };

        let style: Style;
        let title: Title;
        let mut block = Block::bordered();

        if self.input_mode == InputMode::Normal {
            title = Title::from(Line::from(vec![" [1] ".blue().bold(), "Search ".bold()]));
            style = Style::default();
            block = block
                .border_set(search_border_set)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);
        } else {
            title = Title::from(Line::from(vec![
                " [1] ".light_red().bold(),
                "Search ".bold(),
            ]));
            style = Style::default().fg(Color::Yellow);
        }

        block
            .title(title.alignment(Alignment::Right))
            .style(style)
            .render(area, buf);
    }
}
