mod player;
mod welcome;
pub use player::Player;
pub use welcome::Welcome;
pub use welcome::WelcomeEvent;

#[derive(Clone)]
pub enum Screen {
    Welcome(welcome::Welcome),
    Player(player::Player),
}
