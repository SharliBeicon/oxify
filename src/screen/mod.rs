mod welcome;
pub use welcome::Welcome;

pub enum Screen {
    Welcome(welcome::Welcome),
}
