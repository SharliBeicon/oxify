mod app;
use std::{fs, io, path::PathBuf, sync::LazyLock};

pub use app::App;
pub mod auth;
pub mod player;
pub mod widgets;
use auth::AuthState;
use std::time::Duration;
use widgets::PopupKind;

#[derive(Debug)]
pub struct PopupContent {
    title: String,
    content: String,
    kind: PopupKind,
}

#[derive(Debug)]
pub enum OxifyEvent {
    Exit,
    LoginAttempt,
    AuthInfo(AuthState),
    Popup(PopupContent),
}

fn get_or_create_oxify_dir() -> io::Result<PathBuf> {
    let current_dir = std::env::current_dir().expect("Can't get the current directory");
    let oxify_dir = current_dir.join("oxify");

    if !oxify_dir.exists() {
        fs::create_dir(&oxify_dir)?;
    }

    Ok(oxify_dir)
}

pub static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build()
});
