use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = oxify::app::App::default().run(&mut terminal);

    ratatui::restore();

    app_result
}
