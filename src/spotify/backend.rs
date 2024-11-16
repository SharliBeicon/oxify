use crate::{spotify, OxifyEvent, OxifyPlayerEvent};
use librespot::{
    core::{spotify_id::SpotifyItemType, SpotifyId},
    playback::{
        audio_backend::BACKENDS,
        config::{AudioFormat, PlayerConfig},
        mixer::{MixerConfig, MIXERS},
        player::Player,
    },
};
use std::sync::mpsc;

pub async fn run(
    access_token: String,
    ope_tx: tokio::sync::broadcast::Sender<OxifyPlayerEvent>,
    oe_tx: mpsc::Sender<OxifyEvent>,
) {
    let session = match spotify::api::get_backend_session(access_token.clone()).await {
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

    // Send the backend event handler to app.
    let bpe_rx = player.get_player_event_channel();
    OxifyEvent::send(&oe_tx, OxifyEvent::BackendPlayerReceiver(bpe_rx));

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
                assert!(spotify_id.item_type == SpotifyItemType::Track);

                player.load(spotify_id, true, 0);
            }
        }
    }

    OxifyEvent::send(&oe_tx, OxifyEvent::ActiveBackend(false));
}
