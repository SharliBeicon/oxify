use std::{error::Error, io};

use librespot::{
    core::{
        spotify_id::{self, SpotifyItemType},
        Session, SessionConfig, SpotifyId,
    },
    discovery::Credentials,
    playback::{
        audio_backend::BACKENDS,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::Player,
    },
};

pub async fn play_uri(token: String, uri: String) {
    let spotify_id = match SpotifyId::from_uri(&uri) {
        Ok(spotify_id) => spotify_id,
        Err(err) => {
            log::error!("Can't get uri: {err}");
            return;
        }
    };

    let credentials = Credentials::with_access_token(token);
    let session = Session::new(SessionConfig::default(), None);
    if let Err(err) = session.connect(credentials, false).await {
        log::error!("Cannot init a playback connection: {err}");
    }

    let player = Player::new(
        PlayerConfig::default(),
        session,
        Box::new(NoOpVolume),
        move || (BACKENDS[0].1)(None, AudioFormat::default()),
    );

    match spotify_id.item_type {
        SpotifyItemType::Track => player.load(spotify_id, true, 0),
        _ => {
            log::error!("Unsuported spotify id format");
            return;
        }
    }

    let mut rx = player.get_player_event_channel();

    while let Some(event) = rx.recv().await {}
}
