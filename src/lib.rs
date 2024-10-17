pub mod app;
pub mod auth;
pub mod widgets;

pub enum Event {
    Exit,
    Update,
    LoginAttempt,
}
