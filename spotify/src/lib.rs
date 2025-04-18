#![allow(dead_code)]

use anyhow::{Result, anyhow};
use data::{Config, environment};
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
use std::str::FromStr;
use thiserror::Error;

pub mod auth;

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

pub struct Setup {
    format: AudioFormat,
    backend: SinkBuilder,
    device: Option<String>,
    mixer: MixerFn,
    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    mixer_config: MixerConfig,
    credentials: Option<Credentials>,
}

impl Setup {
    pub async fn new(config: &Config) -> Result<Self> {
        let format = AudioFormat::from_str(&config.audio.format)
            .map_err(|_| anyhow!("Audio format not valid"))?;

        let backend =
            audio_backend::find(None).ok_or(anyhow!("Error while loading audio backend"))?;

        let device = Some(String::from(DEVICE));

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

        let credentials = cache.as_ref().and_then(Cache::credentials);

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
