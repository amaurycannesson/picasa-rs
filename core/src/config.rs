use std::path::PathBuf;

use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub clip_model: ClipModelConfig,
    pub face_detection_server: FaceDetectionServerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClipModelConfig {
    pub dir: String,
    pub safetensors_file: String,
    pub tokenizer_file: String,
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FaceDetectionServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let builder = ConfigBuilder::builder()
            .add_source(Self::default_config())
            .add_source(File::from(Self::user_config_path()).required(false))
            .add_source(File::from(Self::specific_config_path()).required(false))
            .add_source(
                Environment::with_prefix("PICASA")
                    .prefix_separator("__")
                    .separator("__"),
            )
            .build()?;

        builder.try_deserialize()
    }

    fn default_config() -> ConfigBuilder {
        let default_toml = include_str!("../../config/default.toml");

        ConfigBuilder::builder()
            .add_source(File::from_str(default_toml, config::FileFormat::Toml))
            .build()
            .expect("Failed to build default config")
    }

    fn user_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("picasa-rs")
            .join("config.toml")
    }

    fn specific_config_path() -> PathBuf {
        match std::env::var("PICASA_CONFIG_FILE") {
            Ok(var) => PathBuf::from(var),
            Err(_) => PathBuf::from("."),
        }
    }
}
