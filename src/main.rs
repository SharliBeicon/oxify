use data::{config::Config, environment, font};
use oxify::Oxify;
use std::env;

mod logger;
mod oxify;
mod screen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next();

    let version = args.next().is_some_and(|s| s == "--version" || s == "-v");

    if version {
        println!("Oxify {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let is_debug = cfg!(debug_assertions);

    let log_stream = logger::setup(is_debug).expect("Cannot setup logging");
    log::info!("Oxify {} started", env!("CARGO_PKG_VERSION"));
    log::info!("config dir: {:?}", environment::config_dir());
    log::info!("data dir: {:?}", environment::data_dir());

    let config = Config::load();
    log::info!("Config loaded: {config:?}");

    let settings = iced::Settings {
        default_font: font::MONO.clone().into(),
        default_text_size: config.font_size.into(),
        id: None,
        antialiasing: false,
        fonts: font::load(),
    };

    iced::daemon("Oxify", Oxify::update, Oxify::view)
        .theme(Oxify::theme)
        .settings(settings)
        .run_with(move || Oxify::new(config, log_stream))
        .inspect_err(|err| log::error!("{}", err))?;

    Ok(())
}
