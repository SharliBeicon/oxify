use super::centered_height;
use crate::OxifyEvent;
use crossterm::event::{Event, KeyCode, KeyEventKind};
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
    Frame,
};
use std::sync::mpsc::Sender;

#[derive(Debug, Default, Clone)]
pub struct AwaitLogin {
    pub event_tx: Option<Sender<OxifyEvent>>,
}

impl AwaitLogin {
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.clone(), frame.area());
    }

    pub fn handle_events(&self, terminal_event: &Option<Event>) {
        let event_tx = self
            .event_tx
            .clone()
            .expect("Event sender not initialized somehow");
        if let Some(crossterm::event::Event::Key(key_event)) = terminal_event {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
                if let Err(err) = event_tx.send(OxifyEvent::Exit) {
                    log::error!("Cannot send event to main app: {err}")
                }
            }
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
