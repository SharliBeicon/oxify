use std::{rc::Rc, sync::mpsc::Sender};

use crossterm::event::{Event as TerminalEvent, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    Frame,
};

use crate::{model::user_profile::UserProfile, Focus, OxifyEvent};

use super::{
    library::Library,
    player::{Player, SearchFullData, SubpanelFocus},
    search::Search,
    tables::{AlbumTable, TrackTable},
    InputMode,
};

#[derive(Default, Debug)]
pub struct MainWindow {
    player: Player,
    library: Library,
    search: Search,
    event_tx: Option<Sender<OxifyEvent>>,

    pub user_profile: Option<UserProfile>,
}

impl MainWindow {
    pub fn set_event_sender(&mut self, tx: Sender<OxifyEvent>) {
        self.event_tx = Some(tx.clone());
        self.player.event_tx = Some(tx.clone());
        self.library.event_tx = Some(tx.clone());
        self.search.event_tx = Some(tx.clone());
    }

    fn draw_input(&self, frame: &mut Frame, area: Rect) {
        #[allow(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            area.x + self.search.character_index as u16 + 1,
            area.y + 1,
        ));
    }

    pub fn handle_events(
        &mut self,
        terminal_event: &Option<TerminalEvent>,
        oxify_event: &Option<OxifyEvent>,
    ) {
        if let Some(oxify_event) = oxify_event {
            match oxify_event {
                OxifyEvent::Focus(focus) => self.set_focus(focus),
                OxifyEvent::InputMode(input_mode) => self.search.input_mode = *input_mode,
                OxifyEvent::SearchResponse(response) => {
                    self.player.search_data = Some(SearchFullData {
                        data: response.clone(),
                        track_table: TrackTable::new(response.clone().tracks.unwrap()),
                        album_table: AlbumTable::new(response.clone().albums.unwrap()),
                    });
                    self.search.reset_cursor();
                    if let Some(event_tx) = &self.event_tx {
                        OxifyEvent::send(event_tx, OxifyEvent::Focus(Focus::Player));
                    }
                }
                _ => (),
            }
        }
        if let Some(terminal_event) = terminal_event {
            if let crossterm::event::Event::Key(key_event) = terminal_event {
                if key_event.kind == KeyEventKind::Press {
                    let event_tx = self
                        .event_tx
                        .clone()
                        .expect("Event sender not initialized somehow");
                    match key_event.code {
                        KeyCode::Esc => {
                            if self.search.input_mode != InputMode::Insert {
                                self.set_focus(&Focus::None)
                            } else {
                                self.search.handle_events(&key_event.code);
                            }
                        }
                        KeyCode::Char('q') => {
                            if let Err(err) = event_tx.send(OxifyEvent::Exit) {
                                log::error!("Cannot send event to main app: {err}")
                            }
                        }
                        _ => {
                            if self.search.input_mode == InputMode::Normal {
                                self.search.handle_events(&key_event.code);
                                self.library.handle_events(&key_event.code);
                                self.player.handle_events(&key_event.code);
                            } else {
                                self.search.handle_events(&key_event.code);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.player.username = self
            .user_profile
            .as_ref()
            .map_or_else(|| "".to_string(), |up| up.display_name.clone());

        let (library_area, search_and_player_area) = layout(frame.area());

        self.library.draw(frame, library_area);
        self.search
            .focused
            .then(|| self.draw_input(frame, search_and_player_area[0]));
        self.search.draw(frame, search_and_player_area[0]);
        self.player.draw(frame, search_and_player_area[1]);
    }

    fn set_focus(&mut self, focus_event: &Focus) {
        match focus_event {
            Focus::Search => {
                self.search.focused = true;
                self.library.focused = false;
                self.player.focused = false;
                self.player.subpanel_focus = SubpanelFocus::None;
            }
            Focus::Library => {
                self.search.focused = false;
                self.search.input_mode = InputMode::Normal;
                self.library.focused = true;
                self.player.focused = false;
                self.player.subpanel_focus = SubpanelFocus::None;
            }
            Focus::Player => {
                self.search.focused = false;
                self.search.input_mode = InputMode::Normal;
                self.library.focused = false;
                self.player.focused = true;
            }
            Focus::None => {
                self.search.focused = false;
                self.search.input_mode = InputMode::Normal;
                self.library.focused = false;
                self.player.focused = false;
                self.player.subpanel_focus = SubpanelFocus::None;
            }
        };
    }
}

fn layout(area: Rect) -> (Rect, Rc<[Rect]>) {
    let search_bar_percentage = match area.height {
        0..20 => 14,
        20..30 => 12,
        30.. => 8,
    };
    let library_percentage = match area.width {
        0..150 => 25,
        150.. => 20,
    };
    let left_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(library_percentage),
            Constraint::Percentage(100 - library_percentage),
        ])
        .split(area);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(search_bar_percentage),
            Constraint::Percentage(100 - search_bar_percentage),
        ])
        .split(left_layout[1]);

    (left_layout[0], right_layout)
}
