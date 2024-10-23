use std::io;

use super::AuthState;
use crate::AGENT;
use base64::prelude::*;
use serde::Deserialize;
use ureq::Response;

#[allow(dead_code)]
#[derive(Deserialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    scope: Option<String>,
    expires_in: i32,
    refresh_token: String,
}

static OAUTH_SCOPES: &str = "playlist-modify playlist-modify-private playlist-modify-public playlist-read playlist-read-collaborative playlist-read-private streaming user-follow-modify user-follow-read user-library-modify user-library-read user-modify user-modify-playback-state user-modify-private user-personalized user-read-currently-playing user-read-email user-read-play-history user-read-playback-position user-read-playback-state user-read-private user-read-recently-played user-top-read";

pub fn get_tokens(code: String, client_id: &str, secret_id: &str) -> io::Result<AuthState> {
    let secrets_encoded = BASE64_STANDARD.encode(format!("{}:{}", client_id, secret_id));
    match AGENT
        .post("https://accounts.spotify.com/api/token")
        .set(
            "Authorization",
            format!("Basic {}", secrets_encoded).as_str(),
        )
        .send_form(&[
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("scope", OAUTH_SCOPES),
            (
                "redirect_uri",
                "http://localhost:60069/authorization/callback",
            ),
        ]) {
        Err(err) => Err(io::Error::new(io::ErrorKind::PermissionDenied, err)),
        Ok(response) => Ok(parse_response(response)?),
    }
}

pub fn refresh_token(auth_state: &AuthState, client_id: &str) -> io::Result<AuthState> {
    let refresh_token = auth_state.refresh_token.as_ref().ok_or_else(|| {
        io::Error::new(io::ErrorKind::PermissionDenied, "Refresh Token not found")
    })?;

    let response = AGENT
        .post("https://accounts.spotify.com/api/token")
        .send_form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", client_id),
        ])
        .map_err(|err| io::Error::new(io::ErrorKind::PermissionDenied, err))?;

    Ok(parse_response(response)?)
}

fn parse_response(response: Response) -> io::Result<AuthState> {
    let response = response.into_string()?;
    let login_response: LoginResponse = serde_json::from_str(&response)?;

    Ok(AuthState {
        login_state: super::LoginState::In,
        access_token: Some(login_response.access_token),
        refresh_token: Some(login_response.refresh_token),
        expiration_time: Some(login_response.expires_in),
    })
}
