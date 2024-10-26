use crate::{model::user_profile::UserProfile, AGENT};
use std::io;

const BASE_URL: &'static str = "https://api.spotify.com/v1";

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
