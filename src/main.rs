use oxify::{auth::State, widgets::app::App};
use std::io;

fn main() -> io::Result<()> {
    let state = State::new();
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = App::default().run(&mut terminal);

    ratatui::restore();

    app_result
}
