use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "config.toml";
pub const WEBSITE_URL: &str = "https://oxify.rs";

pub fn config_dir() -> PathBuf {
    platform_specific_config_dir()
}

pub fn data_dir() -> PathBuf {
    dirs_next::data_dir()
        .expect("expected valid data dir")
        .join("oxify")
}

pub fn cache_dir() -> PathBuf {
    dirs_next::cache_dir()
        .expect("expected valid cache dir")
        .join("oxify")
}

fn platform_specific_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        xdg_config_dir().unwrap_or_else(|| {
            dirs_next::config_dir()
                .expect("expected valid config dir")
                .join("oxify")
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        dirs_next::config_dir()
            .expect("expected valid config dir")
            .join("oxify")
    }
}

#[cfg(target_os = "macos")]
fn xdg_config_dir() -> Option<PathBuf> {
    let config_dir =
        xdg::BaseDirectories::with_prefix("oxify").find_config_file(CONFIG_FILE_NAME)?;

    config_dir.parent().map(|p| p.to_path_buf())
}
