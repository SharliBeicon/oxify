use std::{
    sync::{mpsc::Sender, LazyLock},
    time::Duration,
};

use model::track_data::SearchData;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use tokio::sync::broadcast;
use widgets::{popup::Popup, InputMode};

pub mod app;
pub mod auth;
pub mod model;
pub mod spotify;
pub mod widgets;

pub static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build()
});

#[derive(Clone, Debug)]
pub enum OxifyPlayerEvent {
    PlayTrack(String),
}

impl OxifyPlayerEvent {
    pub fn send(tx: &broadcast::Sender<Self>, event: Self) {
        if let Err(err) = tx.send(event) {
            log::error!("Cannot send event to main app: {err}")
        }
    }
}

#[derive(Debug)]
pub enum OxifyEvent {
    Exit,
    Focus(Focus),
    LoginAttempt,
    SearchRequest(String),
    SearchResponse(Box<SearchData>),
    InputMode(InputMode),
    Popup(Popup<'static>),
    ClosePopup,
    ActiveBackend(bool),
}

impl OxifyEvent {
    pub fn send(tx: &Sender<Self>, event: Self) {
        if let Err(err) = tx.send(event) {
            log::error!("Cannot send event to main app: {err}")
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Focus {
    Search,
    Library,
    Player,
    #[default]
    None,
}

pub fn resize_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
