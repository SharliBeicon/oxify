use widgets::PopupKind;
pub mod app;
pub mod auth;
pub mod widgets;

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
    Popup(PopupContent),
}
