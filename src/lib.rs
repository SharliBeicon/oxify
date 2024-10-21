mod app;
pub use app::App;
pub mod auth;
pub mod player;
pub mod widgets;
use auth::AuthState;
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
