use super::ChannelMessage;
use actix_web::{get, web, HttpResponse, HttpServer};
use serde::Deserialize;
use std::{io, sync::Arc};
use tokio::sync::broadcast::Sender;

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
        let _ = tx.send(ChannelMessage::Code(code.to_string()));
        return HttpResponse::Ok().body(format!("Authorization code: {}", code));
    }
    if let Some(error) = &query.error {
        let _ = tx.send(ChannelMessage::Error(error.to_string()));
        return HttpResponse::BadRequest().body(format!("Error: {}", error));
    }
    let _ = tx.send(ChannelMessage::Error("Missing code or error".to_string()));
    return HttpResponse::BadRequest().body("Missing code or error");
}

#[actix_web::main]
pub async fn run_server(tx: Arc<Sender<ChannelMessage>>) -> io::Result<()> {
    let _rx = tx.subscribe();

    HttpServer::new(move || actix_web::App::new().app_data(web::Data::new(Arc::clone(&tx))))
        .bind("127.0.0.1:60069")?
        .run()
        .await
}
