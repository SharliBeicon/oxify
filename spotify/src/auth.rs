use data::messages::OxifyMessage;
use librespot::oauth::OAuthClientBuilder;
use std::fmt::Display;

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

#[derive(Clone, Debug)]
pub enum OAuthError {
    Error(String),
    Undefined,
}

impl Display for OAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthError::Error(err) => write!(f, "{}", err),
            OAuthError::Undefined => write!(f, "Auth token not defined yet"),
        }
    }
}
pub async fn login() -> OxifyMessage {
    let client = match OAuthClientBuilder::new(CLIENT_ID, CALLBACK_URL, OAUTH_SCOPES.to_vec())
        .open_in_browser()
        .with_custom_message(include_str!("../../auth_response.html"))
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            log::error!("Failed login attempt: {err}");
            return OxifyMessage::Token(None);
        }
    };

    OxifyMessage::Token(
        client
            .get_access_token_async()
            .await
            .map_err(|err| {
                log::error!("Failed login attempt: {err}");
            })
            .ok(),
    )
}
