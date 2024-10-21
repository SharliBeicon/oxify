pub mod api;
mod client;
mod config;
mod server;

#[derive(Debug, Clone)]
pub enum HttpMessage {
    AuthCode(String),
    Error(String),
}

#[derive(Debug, Default)]
pub enum LoginState {
    In,
    #[default]
    Out,
    Loading,
}

#[derive(Debug, Default)]
pub struct AuthState {
    pub login_state: LoginState,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expiration_time: Option<i32>,
}
