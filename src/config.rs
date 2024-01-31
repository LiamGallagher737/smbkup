use std::{fs, io, path::PathBuf};

use thiserror::Error;

#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(rename = "backup")]
    pub backups: Vec<Backup>,
    #[serde(rename = "server")]
    pub servers: Vec<Server>,
}

#[derive(serde::Deserialize)]
pub struct Backup {
    pub name: String,
    pub server: String,
    pub source: PathBuf,
    pub destination: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub username: String,
    pub share: String,
}

pub fn load(path: Option<PathBuf>) -> Result<Config, ConfigLoadError> {
    let path = match path {
        Some(path) => path,
        _ => dirs::config_dir()
            .ok_or(ConfigLoadError::ConfigDirectory)?
            .join("smbkup/config.toml"),
    };
    let file = fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&file)?;
    Ok(config)
}

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("Unable to find the config directory for your OS, please provide the path to your config file with --config")]
    ConfigDirectory,
    #[error("Failed to read your config file")]
    ReadingFile(#[from] io::Error),
    #[error("Failed to deserialize your config file\n{0:#?}")]
    Deserializing(#[from] toml::de::Error),
}
