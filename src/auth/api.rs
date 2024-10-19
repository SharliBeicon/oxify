use super::{http::run_server, HttpMessage};
use crate::{widgets::PopupKind, OxifyEvent, PopupContent};
use rand::distributions::{Alphanumeric, DistString};
use tokio::sync::{mpsc, oneshot};

pub async fn init_login(app_tx: mpsc::Sender<OxifyEvent>) {
    let (tx, mut rx) = mpsc::channel::<HttpMessage>(256);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let server_thread = tokio::spawn(run_server(tx.into(), shutdown_rx));

    if let Err(err) = open::that(auth_query()) {
        log::error!(
            "Cannot open the browser to initiate the login process: {:?}",
            err
        );
    }

    if let Some(msg) = rx.recv().await {
        match msg {
            HttpMessage::Code(code) => {
                if let Err(err) = app_tx
                    .send(OxifyEvent::Popup(PopupContent {
                        title: " Authorization Token ".to_string(),
                        content: code.to_string(),
                        kind: PopupKind::Info,
                    }))
                    .await
                {
                    log::error!("Error while sending event: {}", err);
                }
            }
            HttpMessage::Error(_) => (),
            _ => (),
        }

        if let Err(err) = shutdown_tx.send(()) {
            log::error!(
                "Error sending a shutdown signal to temporary http server: {:?}",
                err
            )
        }
        if let Err(err) = server_thread.await {
            log::error!("Error joining temporary http server thread: {:?}", err)
        }
    }
}

fn auth_query() -> String {
    let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let client_id = "a4df561fbabb40a3b3ead45196990b6d";
    let response_type = "code";
    let redirect_uri = "http://localhost:60069/authorization/callback";

    format!("https://accounts.spotify.com/authorize?client_id={}&response_type={}&redirect_uri={}&state={}", client_id, response_type, redirect_uri, state )
}
