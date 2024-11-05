use ratatui::{
    layout::{Constraint, Rect},
    prelude::Margin,
    style::{palette::tailwind, Color, Modifier, Style},
    text::Text,
    widgets::{
        Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
    Frame,
};

use crate::model::track_data::{AlbumCollection, TrackCollection};

const ITEM_HEIGHT: usize = 4;

#[derive(Debug, Clone)]
struct TableColors {
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
}
impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            header_bg: color.c800,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c600,
            normal_row_color: tailwind::NEUTRAL.c950,
            alt_row_color: tailwind::NEUTRAL.c900,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    table_state: TableState,
    scroll_state: ScrollbarState,
    colors: TableColors,
}

#[derive(Debug, Clone)]
struct TrackData {
    name: String,
    artist: String,
    album: String,
    duration: String,
}

impl TrackData {
    const fn ref_array(&self) -> [&String; 4] {
        [&self.name, &self.artist, &self.album, &self.duration]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn album(&self) -> &str {
        &self.album
    }

    fn duration(&self) -> &str {
        &self.duration
    }
}

#[derive(Debug, Clone)]
pub struct TrackTable {
    state: State,
    items: Vec<TrackData>,
}

impl TrackTable {
    pub fn new(track_collection: TrackCollection) -> Self {
        let items: Vec<TrackData> = track_collection.into();

        Self {
            state: State {
                table_state: TableState::default().with_selected(0),
                scroll_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
                colors: TableColors::new(&tailwind::AMBER),
            },
            items,
        }
    }
    pub fn next_row(&mut self) {
        let i = match self.state.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.table_state.select(Some(i));
        self.state.scroll_state = self.state.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.table_state.select(Some(i));
        self.state.scroll_state = self.state.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.render_table(frame, area);
        self.render_scrollbar(frame, area);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.state.colors.header_fg)
            .bg(self.state.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.state.colors.selected_row_style_fg);

        let header = ["Name", "Artist", "Album", "Duration"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().map(|data| {
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::default())
                .height(2)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(25),
                Constraint::Percentage(15),
            ],
        )
        .header(header)
        .highlight_style(selected_row_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state.table_state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.state.scroll_state,
        );
    }
}

impl From<TrackCollection> for Vec<TrackData> {
    fn from(value: TrackCollection) -> Self {
        value
            .items
            .iter()
            .map(|track_item| TrackData {
                name: track_item.name.clone(),
                artist: track_item.artists[0].name.clone(),
                album: track_item.album.name.clone(),
                duration: millis_to_mm_ss(track_item.duration_ms),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct AlbumData {
    name: String,
    artist: String,
    year: String,
    num_songs: String,
}

impl AlbumData {
    const fn ref_array(&self) -> [&String; 4] {
        [&self.name, &self.artist, &self.year, &self.num_songs]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn year(&self) -> &str {
        &self.year
    }

    fn num_songs(&self) -> &str {
        &self.num_songs
    }
}

#[derive(Debug, Clone)]
pub struct AlbumTable {
    state: State,
    items: Vec<AlbumData>,
}

impl AlbumTable {
    pub fn new(album_collection: AlbumCollection) -> Self {
        let items: Vec<AlbumData> = album_collection.into();

        Self {
            state: State {
                table_state: TableState::default().with_selected(0),
                scroll_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
                colors: TableColors::new(&tailwind::LIME),
            },
            items,
        }
    }
    pub fn next_row(&mut self) {
        let i = match self.state.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.table_state.select(Some(i));
        self.state.scroll_state = self.state.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.table_state.select(Some(i));
        self.state.scroll_state = self.state.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.render_table(frame, area);
        self.render_scrollbar(frame, area);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.state.colors.header_fg)
            .bg(self.state.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.state.colors.selected_row_style_fg);

        let header = ["Name", "Artist", "Year", "Num Tracks"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().map(|data| {
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::default())
                .height(2)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(25),
                Constraint::Percentage(15),
            ],
        )
        .header(header)
        .highlight_style(selected_row_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state.table_state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.state.scroll_state,
        );
    }
}

impl From<AlbumCollection> for Vec<AlbumData> {
    fn from(value: AlbumCollection) -> Self {
        value
            .items
            .iter()
            .map(|album_item| AlbumData {
                name: album_item.name.clone(),
                artist: album_item.artists[0].name.clone(),
                year: album_item.release_date.clone(),
                num_songs: album_item.total_tracks.to_string(),
            })
            .collect()
    }
}
fn millis_to_mm_ss(milliseconds: u32) -> String {
    let seconds = milliseconds / 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}
