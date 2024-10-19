use simplelog::*;
use std::fs::File;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("oxify.log").unwrap(),
    )
    .unwrap();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = oxify::app::App::new().run(&mut terminal);

    ratatui::restore();

    app_result
}
