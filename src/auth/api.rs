use std::{
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use super::{
    client::{self, refresh_token},
    config::Config,
    server, HttpMessage,
};
use crate::auth::AuthState;
use rand::distributions::{Alphanumeric, DistString};

const ONE_MINUTE: i32 = 60;

pub fn login(app_tx: Sender<AuthState>) {
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
                match client::get_tokens(code, config.client_id, config.secret_id) {
                    Err(err) => {
                        if let Err(err) = app_tx.send(AuthState::default()) {
                            log::error!(
                                "Error sending login information through the channel: {}",
                                err
                            );
                        }
                        log::error!("Could not complete login process: {}", err)
                    }
                    Ok(auth_state) => {
                        if let Err(err) = app_tx.send(auth_state.clone()) {
                            log::error!(
                                "Error sending login information through the channel: {}",
                                err
                            );
                        }

                        if server_thread.join().is_err() {
                            log::error!("Error joining server thread");
                        }

                        refresh_task(&auth_state, config.client_id, app_tx);
                    }
                }
                break;
            }
            HttpMessage::Error(err) => {
                log::error!("Error receiving info from server: {}", err)
            }
        }
    }
}

fn refresh_task(auth_state: &AuthState, client_id: &str, app_tx: Sender<AuthState>) {
    let mut expiration_time = match auth_state.expiration_time {
        Some(time) => time,
        None => {
            log::error!("A valid auth state needs an expiration time");
            return;
        }
    };
    loop {
        thread::sleep(Duration::from_secs((expiration_time - ONE_MINUTE) as u64));

        match refresh_token(auth_state, client_id) {
            Ok(new_state) => {
                expiration_time = match new_state.expiration_time {
                    Some(time) => time,
                    None => {
                        log::error!("A valid auth state needs an expiration time");
                        return;
                    }
                };
                if let Err(err) = app_tx.send(new_state.clone()) {
                    log::error!(
                        "Error sending login information through the channel: {}",
                        err
                    );
                }
            }
            Err(err) => {
                log::error!("Cannot refresh auth token: {}", err);
                return;
            }
        }
    }
}
static OAUTH_SCOPES: &[&str] = &[
    "user-read-playback-state",
    "user-modify-playback-state",
    "user-read-currently-playing",
    "app-remote-control",
    "streaming",
    "playlist-read-private",
    "playlist-read-collaborative",
    "playlist-modify-private",
    "playlist-modify-public",
    "user-read-playback-position",
    "user-top-read",
    "user-read-recently-played",
    "user-library-modify",
    "user-library-read",
    "user-read-email",
    "user-read-private",
];

fn auth_query(state: &str, client_id: &str) -> String {
    let response_type = "code";
    let redirect_uri = "http://localhost:60069/authorization/callback";

    format!("https://accounts.spotify.com/authorize?client_id={}&response_type={}&scope={}&redirect_uri={}&state={}", client_id, response_type, &OAUTH_SCOPES.join("%20"), redirect_uri, state)
}
