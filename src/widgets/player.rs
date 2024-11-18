use std::sync::{Arc, Mutex};

use librespot::playback::player::PlayerEvent as BackendPlayerEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Block, Borders, Widget},
    Frame,
};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Default, Clone)]
pub struct Player {
    pub bpe_rx: Option<Arc<Mutex<UnboundedReceiver<BackendPlayerEvent>>>>,
}

impl Player {
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.clone(), area);
    }

    #[allow(unused_variables)]
    pub fn handle_events(&self) {
        if let Some(bpe_rx) = &self.bpe_rx {
            if let Ok(mut bpe_rx) = bpe_rx.lock() {
                if let Ok(backend_event) = bpe_rx.try_recv() {
                    match backend_event {
                        BackendPlayerEvent::PlayRequestIdChanged { play_request_id } => todo!(),
                        BackendPlayerEvent::Stopped {
                            play_request_id,
                            track_id,
                        } => todo!(),
                        BackendPlayerEvent::Loading {
                            play_request_id,
                            track_id,
                            position_ms,
                        } => todo!(),
                        BackendPlayerEvent::Preloading { track_id } => todo!(),
                        BackendPlayerEvent::Playing {
                            play_request_id,
                            track_id,
                            position_ms,
                        } => todo!(),
                        BackendPlayerEvent::Paused {
                            play_request_id,
                            track_id,
                            position_ms,
                        } => todo!(),
                        BackendPlayerEvent::TimeToPreloadNextTrack {
                            play_request_id,
                            track_id,
                        } => todo!(),
                        BackendPlayerEvent::EndOfTrack {
                            play_request_id,
                            track_id,
                        } => todo!(),
                        BackendPlayerEvent::Unavailable {
                            play_request_id,
                            track_id,
                        } => todo!(),
                        BackendPlayerEvent::VolumeChanged { volume } => todo!(),
                        BackendPlayerEvent::PositionCorrection {
                            play_request_id,
                            track_id,
                            position_ms,
                        } => todo!(),
                        BackendPlayerEvent::Seeked {
                            play_request_id,
                            track_id,
                            position_ms,
                        } => todo!(),
                        BackendPlayerEvent::TrackChanged { audio_item } => todo!(),
                        BackendPlayerEvent::SessionConnected {
                            connection_id,
                            user_name,
                        } => todo!(),
                        BackendPlayerEvent::SessionDisconnected {
                            connection_id,
                            user_name,
                        } => todo!(),
                        BackendPlayerEvent::SessionClientChanged {
                            client_id,
                            client_name,
                            client_brand_name,
                            client_model_name,
                        } => todo!(),
                        BackendPlayerEvent::ShuffleChanged { shuffle } => todo!(),
                        BackendPlayerEvent::RepeatChanged { repeat } => todo!(),
                        BackendPlayerEvent::AutoPlayChanged { auto_play } => todo!(),
                        BackendPlayerEvent::FilterExplicitContentChanged { filter } => todo!(),
                    }
                }
            }
        }
    }
}

impl Widget for Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::PLAIN
        };

        Block::bordered()
            .border_set(border_set)
            .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
            .render(area, buf)
    }
}
