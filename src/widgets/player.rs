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

use crate::{model::track_data::SearchData, spotify::backend, Focus, OxifyEvent};

use super::{
    centered_height,
    tables::{AlbumTable, TrackTable},
};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub username: String,
    pub focused: bool,
    pub search_data: Option<SearchFullData>,
    pub event_tx: Option<Sender<OxifyEvent>>,

    pub subpanel_focus: SubpanelFocus,
}

#[derive(Debug, Default, Clone)]
pub enum SubpanelFocus {
    Tracks,
    Albums,
    Artists,
    Playlists,
    #[default]
    None,
}

#[derive(Debug, Clone)]
pub struct SearchFullData {
    pub data: SearchData,
    pub track_table: TrackTable,
    pub album_table: AlbumTable,
}

struct SearchLayout {
    pub albums: Rect,
    pub tracks: Rect,
    pub artists: Rect,
    pub playlists: Rect,
}

impl Player {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.clone(), area);
        if let Some(search_data) = &mut self.search_data {
            let search_layout = search_content_layout(area);
            let margin = Margin {
                horizontal: 1,
                vertical: 1,
            };
            search_data
                .track_table
                .draw(frame, search_layout.tracks.inner(margin));
            search_data
                .album_table
                .draw(frame, search_layout.albums.inner(margin));
        }
    }
    pub fn handle_events(&mut self, key_code: &KeyCode) {
        let event_tx = self
            .event_tx
            .clone()
            .expect("Event sender not initialized somehow");
        if self.focused {
            match key_code {
                KeyCode::Char('t') => self.subpanel_focus = SubpanelFocus::Tracks,
                KeyCode::Char('a') => self.subpanel_focus = SubpanelFocus::Albums,
                KeyCode::Char('r') => self.subpanel_focus = SubpanelFocus::Artists,
                KeyCode::Char('p') => self.subpanel_focus = SubpanelFocus::Playlists,
                _ => (),
            }

            match self.subpanel_focus {
                SubpanelFocus::Tracks => {
                    let search_data = self.search_data.as_mut().expect("Search data is empty");
                    match key_code {
                        KeyCode::Up | KeyCode::Char('k') => search_data.track_table.previous_row(),
                        KeyCode::Down | KeyCode::Char('j') => search_data.track_table.next_row(),
                        KeyCode::Enter => {
                            if let Some(uri) = search_data.track_table.selected_uri() {
                                let event_tx = self
                                    .event_tx
                                    .as_ref()
                                    .expect("Event sender not initialized");
                                OxifyEvent::send(&event_tx, OxifyEvent::PlayUri(uri));
                            }
                        }
                        _ => (),
                    }
                }
                SubpanelFocus::Albums => {
                    let search_data = self.search_data.as_mut().expect("Search data is empty");
                    match key_code {
                        KeyCode::Up | KeyCode::Char('k') => search_data.album_table.previous_row(),
                        KeyCode::Down | KeyCode::Char('j') => search_data.album_table.next_row(),
                        _ => (),
                    }
                }
                SubpanelFocus::Artists => match key_code {
                    _ => (),
                },
                SubpanelFocus::Playlists => match key_code {
                    _ => (),
                },
                SubpanelFocus::None => (),
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
        let mut player_block = Block::bordered().title(
            instructions
                .clone()
                .alignment(Alignment::Right)
                .position(Position::Bottom),
        );

        if !self.focused {
            title = Title::from(Line::from(vec![" [3] ".blue().bold(), "Player ".bold()]));
            player_block = player_block
                .style(Style::default())
                .border_set(player_border_set)
                .borders(Borders::ALL);
        } else {
            title = Title::from(Line::from(vec![
                " [3] ".light_red().bold(),
                "Player ".bold(),
            ]));
            player_block = player_block.style(Style::default().fg(Color::Yellow));
        }

        match self.search_data {
            None => {
                let text_str = format!(
                    " Hello, {}! Use the left window o the search bar to start listening music. ",
                    self.username
                );
                let content = Text::from(text_str.bold());
                player_block = player_block.padding(Padding::top(centered_height(
                    content.height() as u16,
                    &area,
                )));

                player_block = player_block.title(title);

                Paragraph::new(content)
                    .wrap(Wrap { trim: true })
                    .centered()
                    .block(player_block)
                    .render(area, buf);
            }
            Some(_) => {
                let search_layout = search_content_layout(area);
                let mut tracks_title =
                    Title::from(Line::from(vec!["[T]".bold().blue(), "racks".into()]));
                let mut tracks_block = Block::bordered().fg(Color::Yellow);

                let mut albums_title =
                    Title::from(Line::from(vec!["[A]".bold().blue(), "lbums".into()]));
                let mut albums_block = Block::bordered().fg(Color::Yellow);

                let mut artists_title = Title::from(Line::from(vec![
                    "A".into(),
                    "[R]".bold().blue(),
                    "tists".into(),
                ]));
                let mut artists_block = Block::bordered().fg(Color::Yellow);

                let mut playlists_title =
                    Title::from(Line::from(vec!["[P]".bold().blue(), "laylists".into()]));
                let mut playlists_block = Block::bordered().fg(Color::Yellow);

                match self.subpanel_focus {
                    SubpanelFocus::Tracks => {
                        tracks_title =
                            Title::from(Line::from(vec!["[T]".bold().light_red(), "racks".into()]));
                        tracks_block = tracks_block.fg(Color::Cyan);
                    }
                    SubpanelFocus::Albums => {
                        albums_title =
                            Title::from(Line::from(vec!["[A]".bold().light_red(), "lbums".into()]));
                        albums_block = albums_block.fg(Color::Cyan);
                    }
                    SubpanelFocus::Artists => {
                        artists_title = Title::from(Line::from(vec![
                            "A".into(),
                            "[R]".bold().light_red(),
                            "tists".into(),
                        ]));
                        artists_block = artists_block.fg(Color::Cyan);
                    }
                    SubpanelFocus::Playlists => {
                        playlists_title = Title::from(Line::from(vec![
                            "[P]".bold().light_red(),
                            "laylists".into(),
                        ]));
                        playlists_block = playlists_block.fg(Color::Cyan);
                    }
                    SubpanelFocus::None => (),
                }

                player_block.title(title).render(area, buf);
                tracks_block
                    .title(tracks_title)
                    .render(search_layout.tracks, buf);
                albums_block
                    .title(albums_title)
                    .render(search_layout.albums, buf);
                artists_block
                    .title(artists_title)
                    .render(search_layout.artists, buf);
                playlists_block
                    .title(playlists_title)
                    .render(search_layout.playlists, buf);
            }
        }
    }
}

fn search_content_layout(area: Rect) -> SearchLayout {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }));
    let left_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[0]);
    let right_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);

    SearchLayout {
        tracks: left_side[0],
        artists: left_side[1],
        albums: right_side[0],
        playlists: right_side[1],
    }
}
