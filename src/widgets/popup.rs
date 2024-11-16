use crate::{resize_area, OxifyEvent};
use crossterm::event::{Event, KeyCode, KeyEventKind};
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
    Frame,
};
use std::sync::mpsc::Sender;

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
                title_style: Style::new().black(),
                border_style: Style::new().black(),
                style: Style::new().on_light_blue().black(),
            },
            _ => todo!(),
        }
    }
}

impl Popup<'_> {
    pub fn draw(&self, frame: &mut Frame) {
        let popup_area = match frame.area().height {
            0..20 => resize_area(frame.area(), 50, 46),
            20..30 => resize_area(frame.area(), 40, 37),
            30.. => resize_area(frame.area(), 30, 28),
        };

        frame.render_widget(self.clone(), popup_area);
    }

    pub fn handle_events(&self, event_tx: &Sender<OxifyEvent>, terminal_event: &Option<Event>) {
        if let Some(crossterm::event::Event::Key(key_event)) = terminal_event {
            if key_event.kind == KeyEventKind::Press {
                OxifyEvent::send(event_tx, OxifyEvent::ClosePopup);
            }
        }
    }
    pub fn handle_toggle_popup(event_tx: &Sender<OxifyEvent>, terminal_event: &Option<Event>) {
        if let Some(crossterm::event::Event::Key(key_event)) = terminal_event {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('?') {
                OxifyEvent::send(event_tx, OxifyEvent::Popup(help_popup()));
            }
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

pub fn help_popup() -> Popup<'static> {
    let content = Text::from(vec![
        Line::from(vec![
            "<space> ".dark_gray().bold(),
            "Play/Pause Music".into(),
        ]),
        Line::from(vec!["<1/2/3> ".dark_gray().bold(), "Select panel".into()]),
        Line::from(vec![
            "<ESC> ".dark_gray().bold(),
            "Exit focus/insert mode".into(),
        ]),
        Line::from(vec!["<↑ /k> ".dark_gray().bold(), "Move up".into()]),
        Line::from(vec!["<↓ /j> ".dark_gray().bold(), "Move down".into()]),
        Line::from(vec!["<← /h> ".dark_gray().bold(), "Move left".into()]),
        Line::from(vec!["<→ /l> ".dark_gray().bold(), "Move right".into()]),
        Line::from(vec!["<i> ".dark_gray().bold(), "Insert mode".into()]),
        Line::from(vec!["<q> ".dark_gray().bold(), "Exit app".into()]),
    ]);

    Popup {
        title: " Help ".into(),
        content,
        kind: PopupKind::Info,
    }
}
