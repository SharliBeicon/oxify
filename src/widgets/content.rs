use std::sync::mpsc::Sender;
use strum::IntoEnumIterator;

use super::{
    centered_height,
    tables::{AlbumDataTable, ArtistDataTable, PlaylistDataTable, TrackDataTable},
    tabs::SelectedTab,
};
use crate::{model::track_data::SearchData, Focus, OxifyEvent, OxifyPlayerEvent};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding, Paragraph, Tabs, Widget, Wrap,
    },
};
use style::palette::tailwind;
use tokio::sync::broadcast;

#[derive(Default, Clone)]
pub struct Content {
    pub username: String,
    pub focused: bool,
    pub search_data: Option<SearchFullData>,
    pub oe_tx: Option<Sender<OxifyEvent>>,
    pub ope_tx: Option<broadcast::Sender<OxifyPlayerEvent>>,

    pub selected_tab: SelectedTab,
}

#[derive(Debug, Clone)]
pub struct SearchFullData {
    pub data: SearchData,
    pub track_table: Option<TrackDataTable>,
    pub album_table: Option<AlbumDataTable>,
    pub artist_table: Option<ArtistDataTable>,
    pub playlist_table: Option<PlaylistDataTable>,
}

impl Content {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.clone(), area);
        if let Some(search_data) = &mut self.search_data {
            let [_, content_area] =
                Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(area);
            let content_area = content_area.inner(Margin {
                horizontal: 2,
                vertical: 2,
            });
            match self.selected_tab {
                SelectedTab::Tracks => search_data
                    .track_table
                    .as_mut()
                    .expect("TODO")
                    .draw(frame, content_area),
                SelectedTab::Albums => search_data
                    .album_table
                    .as_mut()
                    .expect("TODO")
                    .draw(frame, content_area),
                SelectedTab::Artists => search_data
                    .artist_table
                    .as_mut()
                    .expect("TODO")
                    .draw(frame, content_area),
                SelectedTab::Playlists => search_data
                    .playlist_table
                    .as_mut()
                    .expect("TODO")
                    .draw(frame, content_area),
                SelectedTab::None => (),
            }
        }
    }

    pub fn handle_events(&mut self, key_code: &KeyCode) {
        let oe_tx = self
            .oe_tx
            .clone()
            .expect("Event sender not initialized somehow");
        if self.focused {
            match key_code {
                KeyCode::Char('t') => self.selected_tab = SelectedTab::Tracks,
                KeyCode::Char('a') => self.selected_tab = SelectedTab::Albums,
                KeyCode::Char('r') => self.selected_tab = SelectedTab::Artists,
                KeyCode::Char('p') => self.selected_tab = SelectedTab::Playlists,
                KeyCode::Left | KeyCode::Char('h') => {
                    self.selected_tab = self.selected_tab.previous()
                }
                KeyCode::Right | KeyCode::Char('l') => self.selected_tab = self.selected_tab.next(),
                _ => (),
            }

            match self.selected_tab {
                SelectedTab::Tracks => {
                    let search_data = self.search_data.as_mut().expect("Search data is empty");
                    match key_code {
                        KeyCode::Up | KeyCode::Char('k') => {
                            if let Some(track_table) = &mut search_data.track_table {
                                track_table.previous_row();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(track_table) = &mut search_data.track_table {
                                track_table.next_row();
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(track_table) = &mut search_data.track_table {
                                if let Some(uri) = track_table.selected_uri() {
                                    let ope_tx = self
                                        .ope_tx
                                        .as_ref()
                                        .expect("Player event sender not initialized");
                                    OxifyPlayerEvent::send(
                                        ope_tx,
                                        OxifyPlayerEvent::PlayTrack(uri),
                                    );
                                }
                            }
                        }
                        _ => (),
                    }
                }
                SelectedTab::Albums => {
                    let search_data = self.search_data.as_mut().expect("Search data is empty");
                    match key_code {
                        KeyCode::Up | KeyCode::Char('k') => {
                            if let Some(album_table) = &mut search_data.album_table {
                                album_table.previous_row();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(album_table) = &mut search_data.album_table {
                                album_table.next_row();
                            }
                        }
                        _ => (),
                    }
                }
                SelectedTab::Artists => {
                    let search_data = self.search_data.as_mut().expect("Search data is empty");
                    match key_code {
                        KeyCode::Up | KeyCode::Char('k') => {
                            if let Some(artist_table) = &mut search_data.artist_table {
                                artist_table.previous_row();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(artist_table) = &mut search_data.artist_table {
                                artist_table.next_row();
                            }
                        }
                        _ => (),
                    }
                }
                SelectedTab::Playlists => {
                    let search_data = self.search_data.as_mut().expect("Search data is empty");
                    match key_code {
                        KeyCode::Up | KeyCode::Char('k') => {
                            if let Some(playlist_table) = &mut search_data.playlist_table {
                                playlist_table.previous_row();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(playlist_table) = &mut search_data.playlist_table {
                                playlist_table.next_row();
                            }
                        }
                        _ => (),
                    }
                }
                SelectedTab::None => (),
            }
        } else if let KeyCode::Char('3') = key_code {
            if let Err(err) = oe_tx.send(OxifyEvent::Focus(Focus::Content)) {
                log::error!("Cannot send event to main app: {err}")
            }
        }
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), tailwind::AMBER.c900);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding(" ", " ")
            .divider("|")
            .render(area, buf);
    }
}

impl Widget for Content {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let player_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            bottom_left: symbols::line::NORMAL.horizontal_up,
            bottom_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::PLAIN
        };
        let instructions = Title::from(Line::from(vec![" Help ".into(), "<?> ".blue().bold()]));
        let title: Title;
        let mut player_block = Block::bordered();

        if !self.focused {
            title = Title::from(Line::from(vec![" [3] ".blue().bold(), "Content ".bold()]));
            player_block = player_block
                .style(Style::default())
                .border_set(player_border_set)
                .borders(Borders::ALL);
        } else {
            title = Title::from(Line::from(vec![
                " [3] ".light_red().bold(),
                "Content ".bold(),
            ]));
            player_block = player_block.style(Style::default().fg(Color::Yellow));
        }

        match self.search_data {
            None => {
                let text_str = format!(
                    " Hello, {}! Use the left window or the search bar to start listening music. ",
                    self.username
                );
                let content = Text::from(text_str.bold());
                player_block = player_block.padding(Padding::top(centered_height(
                    content.height() as u16,
                    &area,
                )));

                player_block = player_block.title(title).title(
                    instructions
                        .clone()
                        .alignment(Alignment::Right)
                        .position(Position::Bottom),
                );

                Paragraph::new(content)
                    .wrap(Wrap { trim: true })
                    .centered()
                    .block(player_block)
                    .render(area, buf);
            }
            Some(_) => {
                let [tabs_area, _] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
                    .areas(area.inner(Margin {
                        horizontal: 1,
                        vertical: 2,
                    }));

                player_block.title(title).render(area, buf);
                self.render_tabs(tabs_area, buf);
            }
        }
    }
}
