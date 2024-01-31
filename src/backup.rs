use crate::CliError;
use pavao::{SmbClient, SmbCredentials, SmbMode, SmbOpenOptions, SmbOptions};
use std::{
    io::{Cursor, Write},
    path::PathBuf,
};
use zip::{write::FileOptions, ZipWriter};
use zip_extensions::ZipWriterExtensions;

pub fn run(name: String, config_path: Option<PathBuf>) -> Result<(), CliError> {
    let config = match crate::config::load(config_path) {
        Ok(config) => config,
        Err(err) => {
            println!("{err}");
            std::process::exit(1);
        }
    };

    let Some(backup) = config.backups.iter().find(|b| b.name == name) else {
        return Err(CliError::NoBackupExists { name });
    };

    let Some(share) = config.shares.iter().find(|s| s.name == backup.share) else {
        return Err(CliError::NoShareExists {
            name: backup.name.clone(),
        });
    };

    let password = rpassword::prompt_password("Samba Password: ").unwrap();
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(&share.address)
            .share(&share.share)
            .password(password)
            .username(&share.username),
        SmbOptions::default(),
    )?;

    let _ = client.mkdir(backup.destination.to_str().unwrap(), SmbMode::from(6));

    let date = chrono::Local::now().format(crate::DATE_FORMAT);
    let filename = format!("{name}.{date}.zip");
    let fullpath = backup.destination.join(filename);
    let dest = fullpath.to_str().unwrap();
    let mut writer = client.open_with(dest, SmbOpenOptions::default().create(true).write(true))?;

    let mut buffer = Vec::new();
    let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
    zip.create_from_directory_with_options(&backup.source, FileOptions::default())?;
    drop(zip);

    writer.write_all(&buffer)?;

    println!("Successfully backed up {name}");
    Ok(())
}
