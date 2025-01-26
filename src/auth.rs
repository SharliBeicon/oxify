use crate::oxify::{Message, OAuthError};
use anyhow::anyhow;
use librespot::oauth::OAuthCustomParams;

const OAUTH_SCOPES: [&str; 16] = [
    "user-read-playback-state",
    "user-modify-playback-state",
    "user-read-currently-playing",
    "app-remote-control",
    "streaming",
    "playlist-read-private",
    "playlist-read-collaborative",
    "playlist-modify-private",
    "playlist-modify-public",
    "user-read-playback-position",
    "user-top-read",
    "user-read-recently-played",
    "user-library-modify",
    "user-library-read",
    "user-read-email",
    "user-read-private",
];

pub async fn login() -> Message {
    Message::Token(
        librespot::oauth::get_access_token(
            "a4df561fbabb40a3b3ead45196990b6d",
            "http://localhost:60069/authorization/callback",
            OAUTH_SCOPES.to_vec(),
            Some(OAuthCustomParams {
                open_url: true,
                message: String::from(include_str!("../auth_response.html")),
            }),
        )
        .map_err(|err| OAuthError::Error(err.to_string())),
    )
}
