use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use super::{client, config::Config, server, HttpMessage};
use crate::OxifyEvent;
use rand::distributions::{Alphanumeric, DistString};

pub fn init_login(app_tx: Sender<OxifyEvent>) {
    let config = Config::new();

    let (tx, mut rx) = channel::<HttpMessage>();

    let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let state_clone = state.clone();
    let server_thread = thread::spawn(move || server::run(tx.into(), state_clone));

    if let Err(err) = open::that(auth_query(&state, config.client_id)) {
        log::error!(
            "Cannot open the browser to initiate the login process: {:?}",
            err
        );
    }

    if let Ok(msg) = rx.recv() {
        match msg {
            HttpMessage::Code(code) => match client::finish_login(code, config.secret_id) {
                Ok((access_token, refresh_toke)) => (),
                Err(err) => (),
            },
            HttpMessage::Error(err) => {
                log::error!("Error receiving the authorization code: {}", err)
            }
        }

        if let Err(err) = server_thread.join() {
            log::error!("Error joining temporary http server thread: {:?}", err)
        }
    }
}

fn auth_query(state: &str, client_id: &str) -> String {
    let response_type = "code";
    let redirect_uri = "http://localhost:60069/authorization/callback";

    format!("https://accounts.spotify.com/authorize?client_id={}&response_type={}&redirect_uri={}&state={}", client_id, response_type, redirect_uri, state )
}
