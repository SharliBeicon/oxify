use crate::environment;
use iced::Theme;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_window_size")]
    pub window_size: (f32, f32),
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_size: default_window_size(),
            theme: default_theme(),
            font_family: default_font_family(),
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

fn default_font_family() -> String {
    "Iosevka Term".into()
}

fn default_font_size() -> f32 {
    16.0
}

impl Config {
    pub fn load() -> Self {
        let config_path = environment::config_dir().join(environment::CONFIG_FILE_NAME);

        let mut config_content = String::new();
        log::debug!(
            "Looking for Config file in: {}",
            config_path.to_str().unwrap_or("")
        );

        if let Ok(content) = fs::read_to_string(config_path) {
            config_content = content;
        }

        let config = toml::from_str(&config_content).unwrap_or_else(|err| {
            log::warn!(
                "Config file found but cannot be loaded: {err}\nUsing default config instead"
            );
            Config::default()
        });

        crate::font::set(&config);

        config
    }

    pub fn reload(&mut self) {
        *self = Self::load()
    }
}
