mod appaerance;
mod context;
mod data;
mod logger;
mod oxify;
mod screen;
mod spotify;

use crate::data::font;
use anyhow::Result;
use context::{config::Config, environment};
use oxify::Oxify;
use spotify::Setup;
use std::env;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
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

    font::set();

    let (config, setup) = (|| -> Result<(Config, Setup)> {
        let rt = Runtime::new()?;

        rt.block_on(async {
            let config = Config::load().await;
            let setup = Setup::load(config.clone(), None).await?;

            Ok((config, setup))
        })
    })()?;

    let settings = iced::Settings {
        default_font: font::MONO.clone().into(),
        default_text_size: config.appaerance.font_size.into(),
        id: None,
        antialiasing: false,
        fonts: font::load(),
    };

    iced::daemon("Oxify", Oxify::update, Oxify::view)
        .theme(Oxify::theme)
        .settings(settings)
        .run_with(move || Oxify::new(log_stream, config, setup))
        .inspect_err(|err| log::error!("{}", err))?;

    Ok(())
}
