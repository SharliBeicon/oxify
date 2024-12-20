use super::{
    content::{Content, SearchFullData},
    library::Library,
    player::Player,
    search::Search,
    tables::{AlbumDataTable, ArtistDataTable, PlaylistDataTable, TrackDataTable},
    InputMode,
};
use crate::{model::user_profile::UserProfile, Focus, OxifyEvent, OxifyPlayerEvent};
use crossterm::event::{Event as TerminalEvent, KeyCode, KeyEventKind};
use librespot::playback::player::PlayerEvent as BackendPlayerEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    Frame,
};
use std::{
    rc::Rc,
    sync::{mpsc::Sender, Arc, Mutex},
};
use tokio::sync::{broadcast, mpsc::UnboundedReceiver};

#[derive(Default)]
pub struct MainWindow {
    content: Content,
    library: Library,
    search: Search,
    player: Player,
    oe_tx: Option<Sender<OxifyEvent>>,
    pub user_profile: Option<UserProfile>,
}

impl MainWindow {
    pub fn set_senders(
        &mut self,
        oe_tx: Sender<OxifyEvent>,
        ope_tx: broadcast::Sender<OxifyPlayerEvent>,
    ) {
        self.oe_tx = Some(oe_tx.clone());
        self.content.oe_tx = Some(oe_tx.clone());
        self.library.oe_tx = Some(oe_tx.clone());
        self.search.oe_tx = Some(oe_tx.clone());

        self.content.ope_tx = Some(ope_tx.clone());
    }

    pub fn set_backend_receiver(
        &mut self,
        bpe_rx: Arc<Mutex<UnboundedReceiver<BackendPlayerEvent>>>,
    ) {
        self.player.bpe_rx = Some(bpe_rx);
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
        self.player.handle_events();

        if let Some(oxify_event) = oxify_event {
            match oxify_event {
                OxifyEvent::Focus(focus) => self.set_focus(focus),
                OxifyEvent::InputMode(input_mode) => self.search.input_mode = *input_mode,
                OxifyEvent::SearchResponse(response) => {
                    self.content.search_data = Some(SearchFullData {
                        data: *response.clone(),
                        track_table: response.clone().tracks.map(TrackDataTable::new),
                        album_table: response.clone().albums.map(AlbumDataTable::new),
                        artist_table: response.clone().artists.map(ArtistDataTable::new),
                        playlist_table: response.clone().playlists.map(PlaylistDataTable::new),
                    });
                    self.search.reset_cursor();
                    if let Some(oe_tx) = &self.oe_tx {
                        OxifyEvent::send(oe_tx, OxifyEvent::Focus(Focus::Player));
                    }
                }
                _ => (),
            }
        }
        if let Some(crossterm::event::Event::Key(key_event)) = terminal_event {
            if key_event.kind == KeyEventKind::Press {
                let oe_tx = self
                    .oe_tx
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
                        if let Err(err) = oe_tx.send(OxifyEvent::Exit) {
                            log::error!("Cannot send event to main app: {err}")
                        }
                    }
                    _ => {
                        if self.search.input_mode == InputMode::Normal {
                            self.search.handle_events(&key_event.code);
                            self.library.handle_events(&key_event.code);
                            self.content.handle_events(&key_event.code);
                        } else {
                            self.search.handle_events(&key_event.code);
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.content.username = self
            .user_profile
            .as_ref()
            .map_or_else(|| "".to_string(), |up| up.display_name.clone());

        let (library_area, search_and_content_area, player_area) = layout(frame.area());

        self.library.draw(frame, library_area);
        self.search
            .focused
            .then(|| self.draw_input(frame, search_and_content_area[0]));
        self.search.draw(frame, search_and_content_area[0]);
        self.content.draw(frame, search_and_content_area[1]);
        self.player.draw(frame, player_area);
    }

    fn set_focus(&mut self, focus_event: &Focus) {
        match focus_event {
            Focus::Search => {
                self.search.focused = true;
                self.library.focused = false;
                self.content.focused = false;
            }
            Focus::Library => {
                self.search.focused = false;
                self.search.input_mode = InputMode::Normal;
                self.library.focused = true;
                self.content.focused = false;
            }
            Focus::Player => {
                self.search.focused = false;
                self.search.input_mode = InputMode::Normal;
                self.library.focused = false;
                self.content.focused = false;
            }
            Focus::Content => {
                self.search.focused = false;
                self.search.input_mode = InputMode::Normal;
                self.library.focused = false;
                self.content.focused = true;
            }
            Focus::None => {
                self.search.focused = false;
                self.search.input_mode = InputMode::Normal;
                self.library.focused = false;
                self.content.focused = false;
            }
        };
    }
}

fn layout(area: Rect) -> (Rect, Rc<[Rect]>, Rect) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(area);

    let left_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(outer_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(3), Constraint::Percentage(100)])
        .split(left_layout[1]);

    (left_layout[0], right_layout, outer_layout[1])
}
