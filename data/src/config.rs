use crate::{
    environment,
    messages::{Message, OxifyMessage},
};
use iced::Theme;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { RwLock::new(Config::load().await) })
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_window_size")]
    pub window_size: (f32, f32),
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_size: default_window_size(),
            theme: default_theme(),
            font_size: default_font_size(),
        }
    }
}

impl Config {
    pub fn get_theme(&self) -> Theme {
        match self.theme.as_str() {
            "Light" => Theme::Light,
            "Dark" => Theme::Dark,
            "Dracula" => Theme::Dracula,
            "Nord" => Theme::Nord,
            "Solarized Light" => Theme::SolarizedLight,
            "Solarized Dark" => Theme::SolarizedDark,
            "Gruvbox Light" => Theme::GruvboxLight,
            "Gruvbox Dark" => Theme::GruvboxDark,
            "Catppuccin Latte" => Theme::CatppuccinLatte,
            "Catppuccin Frappé" => Theme::CatppuccinFrappe,
            "Catppuccin Macchiato" => Theme::CatppuccinMacchiato,
            "Catppuccin Mocha" => Theme::CatppuccinMocha,
            "Tokyo Night" => Theme::TokyoNight,
            "Tokyo Night Storm" => Theme::TokyoNightStorm,
            "Tokyo Night Light" => Theme::TokyoNightLight,
            "Kanagawa Wave" => Theme::KanagawaWave,
            "Kanagawa Dragon" => Theme::KanagawaDragon,
            "Kanagawa Lotus" => Theme::KanagawaLotus,
            "Moonfly" => Theme::Moonfly,
            "Nightfly" => Theme::Nightfly,
            "Oxocarbon" => Theme::Oxocarbon,
            "Ferra" => Theme::Ferra,
            _ => Theme::GruvboxDark,
        }
    }
}

fn default_window_size() -> (f32, f32) {
    (800.0, 600.0)
}

fn default_theme() -> String {
    Theme::GruvboxDark.to_string()
}

fn default_font_size() -> f32 {
    16.0
}

impl Config {
    pub async fn load() -> Self {
        let config_dir = environment::config_dir();
        if !config_dir.exists() {
            log::info!("Config directory doesn't exist, creating it.");
            if let Err(err) = tokio::fs::create_dir(&config_dir).await {
                log::warn!("Cannot create config directory: {err}.\nUsing default config.");

                return Config::default();
            }
        }

        let config_path = config_dir.join(environment::CONFIG_FILE_NAME);

        log::debug!(
            "Looking for Config file in: {}",
            config_path.to_str().unwrap_or("")
        );

        match tokio::fs::read_to_string(&config_path).await {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|err| {
                log::warn!(
                    "Config file found but cannot be loaded: {err}\nUsing default config instead"
                );
                Config::default()
            }),
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    let config = Config::default();
                    log::warn!("Config file not found, creating a default one");
                    let content = toml::to_string(&config).unwrap_or(String::new());

                    if let Err(err) = tokio::fs::write(config_path, content).await {
                        log::warn!("Cannot create config file: {err}.\nUsing default config.");
                    }

                    config
                }
                _ => todo!(),
            },
        }
    }

    pub async fn reload(&mut self) -> Message {
        *self = Config::load().await;

        Message::OxifyMessage(OxifyMessage::ConfigReloaded)
    }
}
