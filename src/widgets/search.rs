use super::InputMode;
use crate::{Focus, OxifyEvent};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::Stylize,
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
    Frame,
};
use std::sync::mpsc::Sender;

#[derive(Debug, Default, Clone)]
pub struct Search {
    input: String,
    pub character_index: usize,
    pub focused: bool,
    pub input_mode: InputMode,
    pub oe_tx: Option<Sender<OxifyEvent>>,
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

    pub fn reset_cursor(&mut self) {
        self.input.clear();
        self.character_index = 0;
    }

    fn submit_message(&mut self) -> OxifyEvent {
        OxifyEvent::SearchRequest(self.input.clone())
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.clone(), area);
    }

    pub fn handle_events(&mut self, key_code: &KeyCode) {
        let oe_tx = self
            .oe_tx
            .clone()
            .expect("Event sender not initialized somehow");
        if self.focused {
            if self.input_mode == InputMode::Normal {
                match key_code {
                    KeyCode::Left | KeyCode::Char('h') => self.move_cursor_left(),
                    KeyCode::Right | KeyCode::Char('l') => self.move_cursor_right(),
                    KeyCode::Backspace => self.move_cursor_left(),
                    KeyCode::Char('i') => {
                        OxifyEvent::send(&oe_tx, OxifyEvent::InputMode(InputMode::Insert))
                    }
                    _ => {}
                }
            } else {
                match key_code {
                    KeyCode::Esc => {
                        OxifyEvent::send(&oe_tx, OxifyEvent::InputMode(InputMode::Normal))
                    }
                    KeyCode::Enter => OxifyEvent::send(&oe_tx, self.submit_message()),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Char(to_insert) => self.enter_char(*to_insert),
                    _ => (),
                }
            }
        } else if let KeyCode::Char('1') = key_code {
            OxifyEvent::send(&oe_tx, OxifyEvent::Focus(Focus::Search));
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

        let mode = if self.input_mode == InputMode::Normal {
            " [N]".bold().blue()
        } else {
            " [I]".bold().green()
        };
        if !self.focused {
            title = Title::from(Line::from(vec![" [1] ".blue().bold(), "Search ".bold()]));
            style = Style::default();
            block = block
                .border_set(search_border_set)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);
        } else {
            title = Title::from(Line::from(vec![
                mode,
                " [1] ".light_red().bold(),
                "Search ".bold(),
            ]));
            style = Style::default().fg(Color::Yellow);
        }

        block = block.title(title.alignment(Alignment::Right)).style(style);

        Paragraph::new(self.input.as_str())
            .style(style)
            .block(block)
            .render(area, buf);
    }
}
