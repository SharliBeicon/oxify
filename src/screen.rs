mod welcome;
pub use welcome::Welcome;
pub use welcome::WelcomeEvent;

pub enum Screen {
    Welcome(welcome::Welcome),
    Oxify,
}
