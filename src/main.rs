use chrono::{DateTime, Utc};
use simplelog::*;
use std::fs::OpenOptions;
use std::io;
use std::time::SystemTime;

fn main() -> io::Result<()> {
    let dt: DateTime<Utc> = SystemTime::now().into();
    let filename = dt.format("/tmp/%d-%m-%Y-oxify.log").to_string();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    WriteLogger::init(LevelFilter::Info, Config::default(), file).unwrap();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = oxify::App::new().run(&mut terminal);

    ratatui::restore();

    app_result
}
