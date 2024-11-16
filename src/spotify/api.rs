use crate::{
    model::{track_data::SearchData, user_profile::UserProfile},
    AGENT,
};
use librespot::{
    core::{Session, SessionConfig},
    discovery::Credentials,
};
use std::io::{self, ErrorKind};

const BASE_URL: &str = "https://api.spotify.com/v1";

pub fn get_user_profile(token: String) -> io::Result<UserProfile> {
    match AGENT
        .get(format!("{}/me", BASE_URL).as_str())
        .set("Authorization", format!("Bearer {}", token).as_str())
        .call()
    {
        Err(err) => Err(io::Error::new(io::ErrorKind::PermissionDenied, err)),
        Ok(response) => {
            let response = response.into_string()?;
            let user_profile: UserProfile = serde_json::from_str(&response)?;
            Ok(user_profile)
        }
    }
}

pub fn search(token: String, query: String) -> io::Result<SearchData> {
    match AGENT
        .get(format!("{}/search", BASE_URL).as_str())
        .set("Authorization", format!("Bearer {}", token).as_str())
        .query("q", &query)
        .query("type", "album,track,artist,playlist")
        .query("limit", "50")
        .call()
    {
        Err(err) => Err(io::Error::new(io::ErrorKind::PermissionDenied, err)),
        Ok(response) => {
            let response = response.into_string()?;
            let user_profile: SearchData = serde_json::from_str(&response)?;
            Ok(user_profile)
        }
    }
}

pub async fn get_backend_session(token: String) -> io::Result<Session> {
    let credentials = Credentials::with_access_token(token);
    let session = Session::new(SessionConfig::default(), None);
    if let Err(err) = session.connect(credentials, false).await {
        return Err(io::Error::new(
            ErrorKind::ConnectionRefused,
            format!("Can't get a backend session: {}", err),
        ));
    }

    Ok(session)
}
