use crate::{Focus, OxifyEvent};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::Stylize,
    style::{Color, Style},
    text::Line,
    widgets::{block::Title, Block, Borders, Widget},
    Frame,
};
use std::sync::mpsc::Sender;

#[derive(Debug, Default, Clone)]
pub struct Library {
    pub focused: bool,
    pub oe_tx: Option<Sender<OxifyEvent>>,
}

impl Library {
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.clone(), area);
    }
    pub fn handle_events(&self, key_code: &KeyCode) {
        let oe_tx = self
            .oe_tx
            .clone()
            .expect("Event sender not initialized somehow");
        if self.focused {
            //TODO: Library behavior
            return;
        } else if let KeyCode::Char('2') = key_code {
            if let Err(err) = oe_tx.send(OxifyEvent::Focus(Focus::Library)) {
                log::error!("Cannot send event to main app: {err}")
            }
        }
    }
}

impl Widget for Library {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style: Style;
        let title: Title;
        let mut block = Block::bordered();

        if !self.focused {
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
