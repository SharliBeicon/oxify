use super::{http::run_server, ChannelMessage};
use rand::distributions::{Alphanumeric, DistString};
use tokio::sync::{mpsc, watch};

pub async fn init_login() {
    let (tx, mut rx) = mpsc::channel::<ChannelMessage>(256);
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let client = reqwest::Client::new();

    let server_thread = tokio::spawn(run_server(tx.clone().into(), shutdown_rx));

    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID variable must be present");
    let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let params = [
        ("client_id", client_id.as_str()),
        ("response_type", "code"),
        (
            "redirect_uri",
            "http://127.0.0.1:60069/authorization/callback",
        ),
        ("state", state.as_str()),
    ];

    if let Err(_) = client
        .get("https://accounts.spotify.com/authorize")
        .form(&params)
        .send()
        .await
    {
        panic!("Cannot send login request to Spotify, try again later.");
    }

    if let Some(msg) = rx.recv().await {
        match msg {
            ChannelMessage::Code(_) => (),
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
