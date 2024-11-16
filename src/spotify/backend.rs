use std::sync::mpsc;

use librespot::{
    core::SpotifyId,
    playback::{
        audio_backend::BACKENDS,
        config::{AudioFormat, PlayerConfig},
        mixer::{MixerConfig, MIXERS},
        player::Player,
    },
};

use crate::{OxifyEvent, OxifyPlayerEvent};

pub struct Backend {}

impl Backend {
    pub async fn run(
        access_token: String,
        ope_tx: tokio::sync::broadcast::Sender<OxifyPlayerEvent>,
        oe_tx: mpsc::Sender<OxifyEvent>,
    ) {
        let session = match crate::spotify::api::get_backend_session(access_token.clone()).await {
            Ok(session) => session,
            Err(_) => {
                log::error!("Failing when creating a new backend session");
                OxifyEvent::send(&oe_tx, OxifyEvent::ActiveBackend(false));
                return;
            }
        };
        log::info!("New backend session created");

        let player = Player::new(
            PlayerConfig::default(),
            session,
            (MIXERS[0].1)(MixerConfig::default()).get_soft_volume(),
            move || (BACKENDS[0].1)(None, AudioFormat::default()),
        );
        log::info!("Player attached to session");

        let mut ope_rx = ope_tx.subscribe();

        log::info!("Handling player events");
        while let Ok(event) = ope_rx.recv().await {
            match event {
                OxifyPlayerEvent::PlayTrack(uri) => {
                    let spotify_id = match SpotifyId::from_uri(&uri) {
                        Ok(spotify_id) => spotify_id,
                        Err(err) => {
                            log::error!("Can't get uri: {err}");
                            continue;
                        }
                    };

                    player.load(spotify_id, true, 0);
                }
            }
        }

        OxifyEvent::send(&oe_tx, OxifyEvent::ActiveBackend(false));
    }
}
