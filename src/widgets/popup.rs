use crossterm::event::KeyEvent;
use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::OxifyEvent;

use super::CustomWidget;

#[derive(Debug, Default, Clone)]
pub enum PopupKind {
    #[default]
    Info,
    Error,
    Warning,
}

#[derive(Debug, Default, Setters, Clone)]
pub struct Popup<'a> {
    #[setters(into)]
    pub title: Line<'a>,
    #[setters(into)]
    pub content: Text<'a>,
    pub kind: PopupKind,
}

struct PopupStyle {
    title_style: Style,
    border_style: Style,
    style: Style,
}

impl From<&PopupKind> for PopupStyle {
    fn from(value: &PopupKind) -> Self {
        match value {
            PopupKind::Info => PopupStyle {
                title_style: Style::new().fg(Color::LightBlue),
                border_style: Style::new().bg(Color::Blue),
                style: Style::default().on_black().blue(),
            },
            PopupKind::Error => PopupStyle {
                title_style: todo!(),
                border_style: todo!(),
                style: todo!(),
            },
            PopupKind::Warning => PopupStyle {
                title_style: todo!(),
                border_style: todo!(),
                style: todo!(),
            },
        }
    }
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(PopupStyle::from(&self.kind).title_style)
            .borders(Borders::ALL)
            .border_style(PopupStyle::from(&self.kind).border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(PopupStyle::from(&self.kind).style)
            .block(block)
            .render(area, buf);
    }
}

impl CustomWidget for Popup<'_> {
    fn handle_key_event(&self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match key_event.code {
            _ => Some(OxifyEvent::Exit),
        }
    }
}