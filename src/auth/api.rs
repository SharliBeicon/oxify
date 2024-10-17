use super::{http::run_server, ChannelMessage};
use tokio::sync::{mpsc, watch};

pub async fn init_login() {
    let (tx, mut rx) = mpsc::channel::<ChannelMessage>(256);
    let (shutdown_tx, shutdown_rx) = watch::channel(());

    let server_thread = tokio::spawn(run_server(tx.clone().into(), shutdown_rx));

    if let Some(msg) = rx.recv().await {
        match msg {
            ChannelMessage::Code(_) => {}
            ChannelMessage::Error(_) => {}
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
