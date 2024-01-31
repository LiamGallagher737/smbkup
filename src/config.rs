use crate::CliError;
use std::{fs, path::PathBuf};

#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(rename = "backup")]
    pub backups: Vec<Backup>,
    #[serde(rename = "share")]
    pub shares: Vec<Share>,
}

#[derive(serde::Deserialize)]
pub struct Backup {
    pub name: String,
    pub share: String,
    pub source: PathBuf,
    pub destination: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct Share {
    pub name: String,
    pub address: String,
    pub username: String,
    pub share: String,
}

pub fn load(path: Option<PathBuf>) -> Result<Config, CliError> {
    let path = match path {
        Some(path) => path,
        _ => dirs::config_dir()
            .ok_or(CliError::ConfigDirectory)?
            .join("smbkup/config.toml"),
    };
    let file = fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&file)?;
    Ok(config)
}
