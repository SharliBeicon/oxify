use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use super::{client, server, HttpMessage};
use crate::OxifyEvent;
use rand::distributions::{Alphanumeric, DistString};

pub fn init_login(app_tx: Sender<OxifyEvent>) {
    let (tx, mut rx) = channel::<HttpMessage>();

    let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let state_clone = state.clone();
    let server_thread = thread::spawn(move || server::run(tx.into(), state_clone));

    if let Err(err) = open::that(auth_query(&state)) {
        log::error!(
            "Cannot open the browser to initiate the login process: {:?}",
            err
        );
    }

    if let Ok(msg) = rx.recv() {
        match msg {
            HttpMessage::Code(code) => match client::finish_login(code) {
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

fn auth_query(state: &str) -> String {
    let client_id = "a4df561fbabb40a3b3ead45196990b6d";
    let response_type = "code";
    let redirect_uri = "http://localhost:60069/authorization/callback";

    format!("https://accounts.spotify.com/authorize?client_id={}&response_type={}&redirect_uri={}&state={}", client_id, response_type, redirect_uri, state )
}
