use std::sync::mpsc::Sender;

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

use crate::{auth, OxifyEvent};

use super::centered_height;

#[derive(Debug, Default, Clone)]
pub struct Landing {
    pub auth_tx: Option<Sender<auth::AuthState>>,
    pub event_tx: Option<Sender<OxifyEvent>>,
}

impl Landing {
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.clone(), frame.area());
    }

    pub fn handle_events(&self, terminal_event: &Option<Event>) {
        if let Some(terminal_event) = terminal_event {
            if let crossterm::event::Event::Key(key_event) = terminal_event {
                if key_event.kind == KeyEventKind::Press {
                    let event_tx = self
                        .event_tx
                        .clone()
                        .expect("Event sender not initialized somehow");
                    match key_event.code {
                        KeyCode::Char(' ') => {
                            let auth_tx = self
                                .auth_tx
                                .clone()
                                .expect("Event sender not initialized somehow");
                            if let Err(err) = event_tx.send(OxifyEvent::LoginAttempt) {
                                log::error!("Cannot send event to main app: {err}")
                            }
                            std::thread::spawn(|| auth::api::login(auth_tx));
                        }
                        KeyCode::Char('q') => {
                            if let Err(err) = event_tx.send(OxifyEvent::Exit) {
                                log::error!("Cannot send event to main app: {err}")
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

impl Widget for Landing {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Oxify, a TUI Spotify client ".bold());
        let content = Text::from(
            "
⠀⠀⠀⠀⠀⠀⠀⢀⣠⣤⣤⣶⣶⣶⣶⣤⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀
⠀⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⠀⠀⠀
⠀⢀⣾⣿⡿⠿⠛⠛⠛⠉⠉⠉⠉⠛⠛⠛⠿⠿⣿⣿⣿⣿⣿⣷⡀⠀
⠀⣾⣿⣿⣇⠀⣀⣀⣠⣤⣤⣤⣤⣤⣀⣀⠀⠀⠀⠈⠙⠻⣿⣿⣷⠀
⢠⣿⣿⣿⣿⡿⠿⠟⠛⠛⠛⠛⠛⠛⠻⠿⢿⣿⣶⣤⣀⣠⣿⣿⣿⡄
⢸⣿⣿⣿⣿⣇⣀⣀⣤⣤⣤⣤⣤⣄⣀⣀⠀⠀⠉⠛⢿⣿⣿⣿⣿⡇
⠘⣿⣿⣿⣿⣿⠿⠿⠛⠛⠛⠛⠛⠛⠿⠿⣿⣶⣦⣤⣾⣿⣿⣿⣿⠃
⠀⢿⣿⣿⣿⣿⣤⣤⣤⣤⣶⣶⣦⣤⣤⣄⡀⠈⠙⣿⣿⣿⣿⣿⡿⠀
⠀⠈⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣾⣿⣿⣿⣿⡿⠁⠀
⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠀⠀⠀
⠀⠀⠀⠀⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⠁⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠈⠙⠛⠛⠿⠿⠿⠿⠛⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀

Sp[ox]tify",
        );
        let instructions = Title::from(Line::from(vec![
            " Login ".into(),
            "<space> ".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
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
