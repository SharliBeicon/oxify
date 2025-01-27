use anyhow::Result;
use chrono::{DateTime, Utc};
use log::LevelFilter;
use oxify::Oxify;
use simplelog::WriteLogger;
use std::{env, fs::OpenOptions, time::SystemTime};

fn main() -> Result<()> {
    let dt: DateTime<Utc> = SystemTime::now().into();
    let filename = dt.format("%d-%m-%Y-oxify.log").to_string();
    let temp_dir = env::temp_dir();
    let log_path = temp_dir.join(filename);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), file)?;

    iced::daemon("Oxify", Oxify::update, Oxify::view)
        .run_with(Oxify::new)
        .inspect_err(|err| log::error!("{}", err))?;

    Ok(())
}
