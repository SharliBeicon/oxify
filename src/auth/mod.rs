pub mod api;
pub mod http;

#[derive(Debug, Clone)]
pub enum HttpMessage {
    Ok,
    Code(String),
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
pub struct State {
    pub login_state: LoginState,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
