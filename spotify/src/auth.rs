use data::types::{Message, OAuthError};
use librespot::oauth::OAuthClientBuilder;

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

pub async fn login() -> Message {
    let client = match OAuthClientBuilder::new(CLIENT_ID, CALLBACK_URL, OAUTH_SCOPES.to_vec())
        .open_in_browser()
        .with_custom_message(include_str!("../../auth_response.html"))
        .build()
    {
        Ok(client) => client,
        Err(err) => return Message::Token(Err(OAuthError::Error(err.to_string()))),
    };

    Message::Token(
        client
            .get_access_token_async()
            .await
            .map_err(|err| OAuthError::Error(err.to_string())),
    )
}
