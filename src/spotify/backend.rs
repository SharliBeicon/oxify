use std::sync::Arc;

use librespot::{
    core::{spotify_id::SpotifyItemType, Session, SpotifyId},
    playback::{
        audio_backend::BACKENDS,
        config::{AudioFormat, PlayerConfig},
        mixer::{MixerConfig, MIXERS},
        player::Player,
    },
};

pub struct Backend {
    player: Arc<Player>,
}

impl Backend {
    pub fn new(session: Session) -> Self {
        Self {
            player: Player::new(
                PlayerConfig::default(),
                session,
                (MIXERS[0].1)(MixerConfig::default()).get_soft_volume(),
                move || (BACKENDS[0].1)(None, AudioFormat::default()),
            ),
        }
    }
    pub async fn play_uri(&self, uri: String) {
        let spotify_id = match SpotifyId::from_uri(&uri) {
            Ok(spotify_id) => spotify_id,
            Err(err) => {
                log::error!("Can't get uri: {err}");
                return;
            }
        };

        match spotify_id.item_type {
            SpotifyItemType::Track => self.player.load(spotify_id, true, 0),
            _ => {
                log::error!("Unsuported spotify id format");
                return;
            }
        }

        let mut rx = self.player.get_player_event_channel();

        while let Some(_event) = rx.recv().await {}
    }
}
