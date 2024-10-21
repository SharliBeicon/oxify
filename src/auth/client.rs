use base64::prelude::*;
use serde::Deserialize;
use std::{io, sync::LazyLock, time::Duration};
use ureq::Response;

use super::AuthState;

#[allow(dead_code)]
#[derive(Deserialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    scope: Option<String>,
    expires_in: i32,
    refresh_token: String,
}

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build()
});

pub fn finish_login(code: String, client_id: &str, secret_id: &str) -> io::Result<AuthState> {
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
            (
                "redirect_uri",
                "http://localhost:60069/authorization/callback",
            ),
        ]) {
        Err(err) => Err(io::Error::new(io::ErrorKind::PermissionDenied, err)),
        Ok(response) => Ok(parse_response(response)?),
    }
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
