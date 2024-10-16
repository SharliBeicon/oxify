use std::sync::Arc;

use super::{http::run_server, ChannelMessage};
use tokio::sync::broadcast;

pub fn init_login() {
    let (tx, _rx) = broadcast::channel::<ChannelMessage>(256);

    let tx = Arc::new(tx);
    tokio::spawn(async move { run_server(Arc::clone(&tx)) });
}
