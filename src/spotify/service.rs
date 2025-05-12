use super::setup::Setup;
use librespot::{
    core::Session,
    playback::{mixer::Mixer, player::Player},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Service {
    pub session: Session,
    pub mixer: Arc<dyn Mixer>,
    pub player: Arc<Player>,
}

impl Service {
    pub fn load(setup: Setup) -> Option<Self> {
        let mut session = Session::new(setup.session_config.clone(), setup.cache.clone());

        let mixer = (setup.mixer)(setup.mixer_config.clone());

        let soft_volume = mixer.get_soft_volume();
        let format = setup.format;
        let backend = setup.backend;
        let device = setup.device.clone();

        let player = Player::new(
            setup.player_config.clone(),
            session.clone(),
            soft_volume,
            move || (backend)(device, format),
        );

        if session.is_invalid() {
            session = Session::new(setup.session_config.clone(), setup.cache.clone());
            player.set_session(session.clone());
        }

        Some(Self {
            session,
            mixer,
            player,
        })
    }
}
