use crossterm::event::KeyEvent;
use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear, Paragraph, Widget, Wrap,
    },
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
            _ => PopupStyle {
                title_style: Style::new().black(),
                border_style: Style::new().black(),
                style: Style::new().on_light_blue().black(),
            },
            //PopupKind::Error => PopupStyle {
            //    title_style: todo!(),
            //    border_style: todo!(),
            //    style: todo!(),
            //},
            //PopupKind::Warning => PopupStyle {
            //    title_style: todo!(),
            //    border_style: todo!(),
            //    style: todo!(),
            //},
        }
    }
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title.alignment(Alignment::Center))
            .title(
                Title::from(" Press any key to close ")
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .title_style(PopupStyle::from(&self.kind).title_style)
            .borders(Borders::ALL)
            .border_style(PopupStyle::from(&self.kind).border_style);
        Paragraph::new(self.content.centered())
            .wrap(Wrap { trim: true })
            .centered()
            .style(PopupStyle::from(&self.kind).style)
            .block(block)
            .render(area, buf);
    }
}

impl CustomWidget for Popup<'_> {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<OxifyEvent> {
        match key_event.code {
            _ => Some(OxifyEvent::Exit),
        }
    }
}

pub fn help_popup() -> Popup<'static> {
    let content = Text::from(vec![
        Line::from(vec![
            "<space> ".dark_gray().bold(),
            "Play/Pause Music".into(),
        ]),
        Line::from(vec![
            "<1/2/3> ".dark_gray().bold(),
            "Toggle panel focus".into(),
        ]),
        Line::from(vec!["<ESC> ".dark_gray().bold(), "Defocus panel".into()]),
        Line::from(vec!["<↑ /k> ".dark_gray().bold(), "Move up".into()]),
        Line::from(vec!["<↓ /j> ".dark_gray().bold(), "Move down".into()]),
        Line::from(vec!["<q> ".dark_gray().bold(), "Exit app".into()]),
    ]);

    Popup {
        title: " Help ".into(),
        content,
        kind: PopupKind::Info,
    }
}
