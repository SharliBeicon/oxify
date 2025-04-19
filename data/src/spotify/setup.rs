use crate::{
    Config, environment,
    messages::{Message, OxifyMessage},
};
use anyhow::{Result, anyhow};
use librespot::{
    connect::ConnectConfig,
    core::{SessionConfig, cache::Cache},
    discovery::{Credentials, DeviceType},
    playback::{
        audio_backend::{self, SinkBuilder},
        config::{AudioFormat, Bitrate, PlayerConfig},
        mixer::{self, MixerConfig, MixerFn},
    },
};
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;

const DEVICE: &str = "Oxify";

#[derive(Debug, Error)]
pub enum ParseFileSizeError {
    #[error("empty argument")]
    EmptyInput,
    #[error("invalid suffix")]
    InvalidSuffix,
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseFloatError),
    #[error("non-finite number specified")]
    NotFinite(f64),
}

#[derive(Clone)]
pub struct Setup {
    pub format: AudioFormat,
    pub backend: SinkBuilder,
    pub device: Option<String>,
    pub mixer: MixerFn,
    pub cache: Option<Cache>,
    pub player_config: PlayerConfig,
    pub session_config: SessionConfig,
    pub connect_config: ConnectConfig,
    pub mixer_config: MixerConfig,
    pub credentials: Option<Credentials>,
}

impl Debug for Setup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Setup")
            .field("format", &self.format)
            .field("backend", &self.backend)
            .field("device", &self.device)
            .field("mixer", &self.mixer)
            .field(
                "cache",
                &self.cache.as_ref().map(|_| "Debug not implemented"),
            )
            .field("player_config", &"Debug not implemented")
            .field("session_config", &self.session_config)
            .field("connect_config", &self.connect_config)
            .field("mixer_config", &self.mixer_config)
            .field("credentials", &self.credentials)
            .finish()
    }
}

impl Setup {
    pub async fn load(config: Config, access_token: Option<String>) -> Result<Setup> {
        let format = AudioFormat::from_str(&config.audio.format)
            .map_err(|_| anyhow!("Audio format not valid"))?;

        let backend =
            audio_backend::find(None).ok_or(anyhow!("Error while loading audio backend"))?;

        let device = None; // Default rodio's device

        let mixer = mixer::find(None).ok_or(anyhow!("Error while loading audio mixer"))?;

        let cache = {
            let credentials_path = {
                let cache_dir = environment::cache_dir();
                if !cache_dir.exists() {
                    log::info!("Data directory doesn't exist, creating it.");
                    tokio::fs::create_dir(&cache_dir)
                        .await
                        .ok()
                        .map(|_| cache_dir.clone())
                } else {
                    None
                }
            };

            let data_path = {
                let data_dir = environment::data_dir();
                if !data_dir.exists() {
                    log::info!("Data directory doesn't exist, creating it.");
                    tokio::fs::create_dir(&data_dir)
                        .await
                        .ok()
                        .map(|_| data_dir.clone())
                } else {
                    None
                }
            };

            let limit = parse_file_size(&config.audio.cache_limit_size)?;
            match Cache::new(
                credentials_path.clone(),
                credentials_path,
                data_path,
                Some(limit),
            ) {
                Ok(cache) => Some(cache),
                Err(e) => {
                    log::warn!("Cannot create cache: {}", e);
                    None
                }
            }
        };

        let mut player_config = PlayerConfig::default();
        player_config.bitrate = Bitrate::from_str(&config.audio.bitrate.to_string())
            .map_err(|_| anyhow!("Incorrect bitrate, options are: 96, 160, 320"))?;

        let connect_config = ConnectConfig {
            name: DEVICE.to_string(),
            device_type: DeviceType::Computer,
            initial_volume: config.audio.initial_volume,
            ..Default::default()
        };

        let session_config = SessionConfig {
            device_id: DEVICE.to_string(),
            ..Default::default()
        };

        let mixer_config = MixerConfig::default();

        let credentials = access_token.map_or_else(
            || cache.as_ref().and_then(Cache::credentials),
            |token| Some(Credentials::with_access_token(token)),
        );

        Ok(Self {
            format,
            backend,
            device,
            mixer,
            cache,
            player_config,
            connect_config,
            session_config,
            mixer_config,
            credentials,
        })
    }

    pub async fn reload(config: Config, access_token: String) -> Message {
        let setup = Self::load(config, Some(access_token))
            .await
            .inspect_err(|err| log::error!("{err}"))
            .ok();

        Message::OxifyMessage(OxifyMessage::Setup(setup))
    }
}

fn parse_file_size(input: &str) -> Result<u64> {
    let mut iter = input.chars();
    let mut suffix = iter.next_back().ok_or(ParseFileSizeError::EmptyInput)?;
    let mut suffix_len = 0;

    let iec = matches!(suffix, 'i' | 'I');

    if iec {
        suffix_len += 1;
        suffix = iter.next_back().ok_or(ParseFileSizeError::InvalidSuffix)?;
    }

    let base: u64 = if iec { 1024 } else { 1000 };

    suffix_len += 1;
    let exponent = match suffix.to_ascii_uppercase() {
        '0'..='9' if !iec => {
            suffix_len -= 1;
            0
        }
        'K' => 1,
        'M' => 2,
        'G' => 3,
        'T' => 4,
        'P' => 5,
        'E' => 6,
        'Z' => 7,
        'Y' => 8,
        _ => return Err(ParseFileSizeError::InvalidSuffix.into()),
    };

    let num = {
        let mut iter = input.chars();

        for _ in (&mut iter).rev().take(suffix_len) {}

        iter.as_str().parse::<f64>()?
    };

    if !num.is_finite() {
        return Err(ParseFileSizeError::NotFinite(num).into());
    }

    Ok((num * base.pow(exponent) as f64) as u64)
}
