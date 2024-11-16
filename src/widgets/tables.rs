use crate::model::track_data::{
    AlbumCollection, ArtistCollection, PlaylistCollection, TrackCollection,
};
use oxify_derive::OxifyTable;
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

const ITEM_HEIGHT: usize = 4;

fn generate_constraint_vector(len: usize) -> Vec<Constraint> {
    let percentage = 100 / len as u16;
    (0..len)
        .map(|_| Constraint::Percentage(percentage))
        .collect()
}

fn millis_to_mm_ss(milliseconds: u32) -> String {
    let seconds = milliseconds / 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}

#[derive(Debug, Clone)]
struct TableColors {
    header_bg: Color,
    header_fg: Color,
    selected_row_style_fg: Color,
}
impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            header_bg: color.c800,
            header_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c600,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    table_state: TableState,
    scroll_state: ScrollbarState,
    colors: TableColors,
}

#[derive(Debug, Clone, OxifyTable)]
struct TrackData {
    name: String,
    artist: String,
    album: String,
    duration: String,
    #[skip]
    uri: String,
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
                uri: track_item.uri.clone(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, OxifyTable)]
struct AlbumData {
    name: String,
    artist: String,
    year: String,
    num_tracks: String,
    #[skip]
    uri: String,
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
                num_tracks: album_item.total_tracks.to_string(),
                uri: album_item.uri.to_string(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, OxifyTable)]
struct ArtistData {
    name: String,
    genre: String,
    followers: String,
    #[skip]
    uri: String,
}

impl From<ArtistCollection> for Vec<ArtistData> {
    fn from(value: ArtistCollection) -> Self {
        value
            .items
            .iter()
            .map(|artist_item| ArtistData {
                name: artist_item.name.clone(),
                genre: artist_item
                    .genres
                    .iter()
                    .map(|word| {
                        let mut chars = word.chars();
                        chars
                            .next()
                            .map(|c| c.to_uppercase().collect::<String>())
                            .unwrap_or_default()
                            + chars.as_str()
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
                followers: artist_item
                    .followers
                    .clone()
                    .map_or_else(|| 0.to_string(), |f| f.total.to_string()),
                uri: artist_item.uri.clone(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, OxifyTable)]
struct PlaylistData {
    name: String,
    description: String,
    num_tracks: String,
    #[skip]
    uri: String,
}

impl From<PlaylistCollection> for Vec<PlaylistData> {
    fn from(value: PlaylistCollection) -> Self {
        value
            .items
            .iter()
            .map(|playlist_item| PlaylistData {
                name: playlist_item.name.clone(),
                description: playlist_item.description.clone(),
                num_tracks: playlist_item.tracks.total.to_string(),
                uri: playlist_item.uri.clone(),
            })
            .collect()
    }
}
