use iced::Theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(Config::load()));

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default = "default_window_size")]
    pub window_size: (f32, f32),
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_font_family")]
    pub font_family: String,
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

fn default_font_family() -> String {
    "Arial".into()
}

impl Config {
    pub fn load() -> Self {
        let config_path = get_config_path();

        let config_content = match fs::read_to_string(config_path) {
            Ok(content) => content,
            Err(_) => return Config::default(),
        };

        toml::from_str(&config_content).unwrap_or_else(|_| Config::default())
    }

    pub fn reload(&mut self) {
        *self = Self::load()
    }
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find user home directory");
    path.push(".oxify");
    path.push("config.toml");
    path
}
