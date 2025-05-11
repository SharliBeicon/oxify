pub mod welcome;

pub use welcome::Welcome;
pub use welcome::WelcomeEvent;

#[derive(Clone)]
pub enum Screen {
    Welcome(welcome::Welcome),
}
