use std::sync::mpsc::Sender;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding, Paragraph, Widget, Wrap,
    },
};

use crate::{Focus, OxifyEvent};

use super::centered_height;

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub username: String,
    pub focused: bool,

    pub event_tx: Option<Sender<OxifyEvent>>,
}

impl Player {
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.clone(), frame.area());
    }
    pub fn handle_events(&self, key_code: &KeyCode) {
        let event_tx = self
            .event_tx
            .clone()
            .expect("Event sender not initialized somehow");
        if self.focused {
            match key_code {
                _ => (),
            }
        } else {
            match key_code {
                KeyCode::Char('3') => {
                    if let Err(err) = event_tx.send(OxifyEvent::Focus(Focus::Player)) {
                        log::error!("Cannot send event to main app: {err}")
                    }
                }
                _ => (),
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

        if !self.focused {
            title = Title::from(Line::from(vec![" [3] ".blue().bold(), "Player ".bold()]));
            block = block
                .style(Style::default())
                .border_set(player_border_set)
                .borders(Borders::ALL);
        } else {
            title = Title::from(Line::from(vec![
                " [3] ".light_red().bold(),
                "Player ".bold(),
            ]));
            block = block.style(Style::default().fg(Color::Yellow));
        }

        block = block.title(title);
        Paragraph::new(content)
            .wrap(Wrap { trim: true })
            .centered()
            .block(block)
            .render(area, buf);
    }
}
