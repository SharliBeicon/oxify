pub struct Config {
    pub client_id: &'static str,
    pub secret_id: &'static str,
}

impl Config {
    pub fn new() -> &'static Self {
        Box::leak(Box::new(Self {
            client_id: std::env!("CLIENT_ID", "CLIENT_ID env var must be defined"),
            secret_id: std::env!("SECRET_ID", "SECRET_ID env var must be defined"),
        }))
    }
}
