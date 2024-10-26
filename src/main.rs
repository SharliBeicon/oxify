use chrono::{DateTime, Utc};
use oxify::App;
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
    WriteLogger::init(LevelFilter::Info, Config::default(), file)
        .expect("Cannot init logging engine");

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = App::new().run(&mut terminal);

    ratatui::restore();

    app_result
}
