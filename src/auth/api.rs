use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use super::{client, config::Config, server, HttpMessage};
use crate::{auth::AuthState, OxifyEvent};
use rand::distributions::{Alphanumeric, DistString};

pub fn init_login(app_tx: Sender<OxifyEvent>) {
    let config = Config::new();

    let (tx, rx) = channel::<HttpMessage>();

    let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let state_clone = state.clone();
    let server_thread = thread::spawn(move || server::run(tx.into(), state_clone));

    if let Err(err) = open::that(auth_query(&state, config.client_id)) {
        log::error!(
            "Cannot open the browser to initiate the login process: {:?}",
            err
        );
    }

    while let Ok(msg) = rx.recv() {
        match msg {
            HttpMessage::AuthCode(code) => {
                match client::finish_login(code, config.client_id, config.secret_id) {
                    Err(err) => {
                        if let Err(err) = app_tx.send(OxifyEvent::AuthInfo(AuthState::default())) {
                            log::error!(
                                "Error sending login information through the channel: {}",
                                err
                            );
                        }
                        log::error!("Could not complete login process: {}", err)
                    }
                    Ok(auth_state) => {
                        if let Err(err) = app_tx.send(OxifyEvent::AuthInfo(auth_state)) {
                            log::error!(
                                "Error sending login information through the channel: {}",
                                err
                            );
                        }
                    }
                }
                if let Err(_) = server_thread.join() {
                    log::error!("Error joining server thread");
                }
                break;
            }
            HttpMessage::Error(err) => {
                log::error!("Error receiving info from server: {}", err)
            }
        }
    }
}

fn auth_query(state: &str, client_id: &str) -> String {
    let response_type = "code";
    let redirect_uri = "http://localhost:60069/authorization/callback";

    format!("https://accounts.spotify.com/authorize?client_id={}&response_type={}&redirect_uri={}&state={}", client_id, response_type, redirect_uri, state )
}
