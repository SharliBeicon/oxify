use super::ChannelMessage;
use actix_web::{get, web, HttpResponse, HttpServer};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, oneshot};

#[derive(Deserialize)]
struct CallbackQuery {
    code: Option<String>,
    error: Option<String>,
}

#[get("/authorization/callback")]
async fn authorization_callback(
    query: web::Query<CallbackQuery>,
    tx: web::Data<Arc<Sender<ChannelMessage>>>,
) -> HttpResponse {
    if let Some(code) = &query.code {
        let _ = tx.send(ChannelMessage::Code(code.to_string())).await;
        return HttpResponse::Ok().into();
    }
    if let Some(error) = &query.error {
        let _ = tx.send(ChannelMessage::Error(error.to_string())).await;
        return HttpResponse::BadRequest().into();
    }
    let _ = tx
        .send(ChannelMessage::Error("Missing code or error".to_string()))
        .await;
    return HttpResponse::BadRequest().into();
}

pub async fn run_server(tx: Arc<Sender<ChannelMessage>>, shutdown_rx: oneshot::Receiver<()>) -> () {
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(tx.clone()))
            .service(authorization_callback)
    })
    .bind("127.0.0.1:60069")
    .expect("Problem creating an http server")
    .run();

    let server_handle = server.handle();

    tokio::select! {
        _ = shutdown_rx => server_handle.stop(true).await,
        _ = server => {}
    }
}
