use crate::{Focus, OxifyEvent};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding, Paragraph, Widget, Wrap,
    },
};

use super::{centered_height, CustomWidget, InputMode};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub username: String,
    pub input_mode: InputMode,
}

impl CustomWidget for Player {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') => Some(OxifyEvent::Exit),
                KeyCode::Char('3') => {
                    self.input_mode = InputMode::Focus;
                    Some(OxifyEvent::Focus(Focus::Player))
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

impl Widget for Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let player_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            bottom_left: symbols::line::NORMAL.horizontal_up,
            ..symbols::border::PLAIN
        };
        let instructions = Title::from(Line::from(vec![" Help ".into(), "<?> ".blue().bold()]));
        let title: Title;
        let text_str = format!(
            " Hello, {}! Use the left window o the search bar to start listening music. ",
            self.username
        );
        let content = Text::from(text_str.bold());
        let mut block = Block::bordered()
            .title(
                instructions
                    .alignment(Alignment::Right)
                    .position(Position::Bottom),
            )
            .padding(Padding::top(centered_height(
                content.height() as u16,
                &area,
            )));

        match self.input_mode {
            InputMode::Normal => {
                title = Title::from(Line::from(vec![" [3] ".blue().bold(), "Player ".bold()]));
                block = block
                    .style(Style::default())
                    .border_set(player_border_set)
                    .borders(Borders::ALL);
            }
            InputMode::Focus => {
                title = Title::from(Line::from(vec![
                    " [3] ".light_red().bold(),
                    "Player ".bold(),
                ]));
                block = block.style(Style::default().fg(Color::Yellow));
            }
        }

        block = block.title(title);
        Paragraph::new(content)
            .wrap(Wrap { trim: true })
            .centered()
            .block(block)
            .render(area, buf);
    }
}
