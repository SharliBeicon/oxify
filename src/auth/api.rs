use super::{http::run_server, ChannelMessage};
use rand::distributions::{Alphanumeric, DistString};
use tokio::sync::{mpsc, oneshot};

pub async fn init_login() {
    let (tx, mut rx) = mpsc::channel::<ChannelMessage>(256);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let server_thread = tokio::spawn(run_server(tx.into(), shutdown_rx));

    if let Err(err) = open::that(auth_query()) {
        panic!(
            "Cannot open the browser to initiate the login process: {:?}",
            err
        );
    }

    if let Some(msg) = rx.recv().await {
        match msg {
            ChannelMessage::Code(code) => println!("Authorization code: {}", code),
            ChannelMessage::Error(_) => (),
            _ => (),
        }

        if let Err(err) = shutdown_tx.send(()) {
            eprintln!(
                "Error sending a shutdown signal to temporary http server: {:?}",
                err
            )
        }
        if let Err(err) = server_thread.await {
            eprintln!("Error joining temporary http server thread: {:?}", err)
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
