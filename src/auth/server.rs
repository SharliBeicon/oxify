use super::HttpMessage;
use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc::Sender, Arc},
};

pub fn run(tx: Arc<Sender<HttpMessage>>, state: String) {
    let listener = match TcpListener::bind("127.0.0.1:60069") {
        Ok(listener) => listener,
        Err(e) => {
            log::error!("Cannot setup a local connection: {}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                log::error!("Error getting a connection stream: {}", err);
                return;
            }
        };

        match handle_connection(stream, &state) {
            Err(err) => log::warn!("Unrecognized HTTP Callback response: {}", err),
            Ok(code) => {
                if let Err(err) = tx.send(HttpMessage::Code(code)) {
                    log::error!("Error sending callback response to client: {}", err);
                }
                break;
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, state: &str) -> io::Result<String> {
    let buf_reader = BufReader::new(&mut stream);
    let request = if let Some(request) = buf_reader.lines().next() {
        request
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Empty request from authorization API",
        ))
    }?;

    let code = parse_http(request, state)?;

    //TODO: Replace this content with a proper html
    let content = "Gracias Maki, vuelve a la app.";
    let length = content.len();

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {length}\r\n\r\n\
        {content}"
    );
    stream.write_all(response.as_bytes())?;

    Ok(code)
}

fn parse_http(request: String, state: &str) -> io::Result<String> {
    if !request.starts_with("GET /authorization/callback?") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "HTTP Callback endpoint not recognized",
        ));
    }

    let params_string = if let Some(query_start) = request.find('?') {
        Ok(&request[query_start + 1..])
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Params not found in HTTP Callback URI",
        ))
    }?;
    let params_string = params_string.replace(" HTTP/1.1", "");

    let params: Vec<&str> = params_string.split('&').collect();
    let mut code = None;
    let mut received_state = None;

    for param in params {
        let mut key_value = param.split('=');
        if let Some(key) = key_value.next() {
            if let Some(value) = key_value.next() {
                if key == "code" {
                    code = Some(value);
                }
                if key == "state" {
                    received_state = Some(value);
                }
            }
        }
    }

    match (code, received_state) {
        (Some(code), Some(received_state)) => {
            if state != received_state {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "States not matching",
                ));
            }

            Ok(code.to_string())
        }
        (_, _) => Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Response does not have expected format",
        )),
    }
}
