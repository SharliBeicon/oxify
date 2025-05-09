use data::messages::{Message, OxifyMessage};
use librespot::oauth::OAuthClientBuilder;
use thiserror::Error;

const CLIENT_ID: &str = "a4df561fbabb40a3b3ead45196990b6d";
const CALLBACK_URL: &str = "http://localhost:60069/authorization/callback";
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

pub use librespot::oauth::OAuthToken;
#[derive(Debug, Error)]
pub enum OAuthError {
    #[error(transparent)]
    Error(#[from] librespot::oauth::OAuthError),
    #[error("Auth token not defined yet")]
    Undefined,
}

pub async fn login() -> Message {
    let client = match OAuthClientBuilder::new(CLIENT_ID, CALLBACK_URL, OAUTH_SCOPES.to_vec())
        .open_in_browser()
        .with_custom_message(include_str!("../../auth_response.html"))
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            log::error!("Failed login attempt: {err}");
            return Message::OxifyMessage(OxifyMessage::Token(None));
        }
    };

    Message::OxifyMessage(OxifyMessage::Token(
        client
            .get_access_token_async()
            .await
            .map_err(|err| {
                log::error!("Failed login attempt: {err}");
            })
            .ok(),
    ))
}
