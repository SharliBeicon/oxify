use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchData {
    pub tracks: Option<TrackCollection>,
    pub artists: Option<ArtistCollection>,
    pub albums: Option<AlbumCollection>,
    pub playlists: Option<PlaylistCollection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackCollection {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<TrackItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackItem {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistCollection {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<ArtistItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistItem {
    pub external_urls: ExternalUrls,
    pub followers: Option<Followers>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumCollection {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<Album>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistCollection {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<PlaylistItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: TrackReference,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackReference {
    pub href: String,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub album_type: String,
    pub total_tracks: u32,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub uri: String,
    pub artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Followers {
    pub href: Option<String>,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}
