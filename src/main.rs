use std::path::PathBuf;

use clap::Parser;

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

fn main() {
    let cli = Cli::parse();
    match cli {
        Cli::Backup { name, config } => backup::run(name, config),
        Cli::Restore { name } => restore::run(name),
    }
}
