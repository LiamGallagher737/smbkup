use clap::Parser;
use std::{path::PathBuf, process::ExitCode};
use thiserror::Error;

mod backup;
mod config;
mod restore;

const DATE_FORMAT: &str = "%Y-%m-%d-%H-%M-%S%z";

#[derive(Parser)]
#[command(version)]
enum Cli {
    /// Run a backup
    #[command(alias = "b")]
    Backup {
        /// The name of the backup you want to run
        name: String,
        /// The path you your config (optional)
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    #[command(alias = "r")]
    /// Restore a backup
    Restore {
        /// The name of the backup you want to restore
        name: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli {
        Cli::Backup { name, config } => backup::run(name, config),
        Cli::Restore { name } => restore::run(name),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Error, Debug)]
enum CliError {
    #[error("Unable to find the config directory for your OS, please provide the path to your config file with --config")]
    ConfigDirectory,
    #[error("IO Error: {0}")]
    ReadingConfig(#[from] std::io::Error),
    #[error("Failed to deserialize your config file: {0}")]
    DeserializingConfig(#[from] toml::de::Error),
    #[error("No backups exist in the config file named {name}")]
    NoBackupExists { name: String },
    #[error("No share exist in the config file named {name}")]
    NoShareExists { name: String },
    #[error("Something went wrong with smb: {0}")]
    Smb(#[from] pavao::SmbError),
    #[error("Failed to zip the source directory: {0}")]
    Zip(#[from] zip::result::ZipError),
}
