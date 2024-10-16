pub mod api;
pub mod http;

#[derive(Debug, Default)]
pub struct State {
    pub logged: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            logged: false,
            access_token: None,
            refresh_token: None,
        }
    }
}
