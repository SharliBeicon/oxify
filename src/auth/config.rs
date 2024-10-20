use std::env;

pub struct Config {
    pub client_id: &'static str,
    pub secret_id: &'static str,
}

impl Config {
    pub fn new() -> &'static Self {
        Box::leak(Box::new(Self {
            client_id: env::var("CLIENT_ID")
                .expect("CLIENT_ID env var must be defined")
                .leak(),
            secret_id: env::var("SECRET_ID")
                .expect("SECRET_ID env var must be defined")
                .leak(),
        }))
    }
}
