<div align="center">

# smbkup (Work in progress)

A cli program for backing up local directories to a Samba share

</div>

## Usage

Servers and backups are defined in a `config.toml` file at the following paths.

- Linux: `~/.config/smbkup/config.toml`
- MacOS: `~/Library/Application Support/smbkup/config.toml`

Here's an example config for a single backup of your `.config` directory

```toml
[[server]]
name = "Home Server" # The name of this server to be used when defining backups
address = "smb://192.168.1.100" # The address of this server which may include a port
username = "Joe" # The username to login with, the password will be promted for when running commands
share = "Backups" # The share on the server to use

[[backup]]
name = "config" # The name of this backup to be used with the `smbkup backup <name>` command
server = "Home Server" # The server for this backup to be uploaded to
source = "/home/joe/.config" # The directory to be backed up
destination = "/joe/config" # The location of the share to store the backups
```

You could then run a backup of your `.config` directory with `smbkup backup config`, you will then be promted for the shares password.

## Installation

Currently the only way to install this is with cargo.

```
cargo install --git https://github.com/LiamGallagher737/smbkup
```
### Linux dependencies

The program depends on `libsmbclient` at runtime and `libsmbclient-dev` or `libsmbclient-devel` for building depending on your distro.

#### Red Hat

```
dnf install libsmbclient libsmbclient-devel
```

#### Debian / Ubuntu

```
apt install libsmbclient libsmbclient-dev
```

### MacOS dependencies

Samba must be installed

```
brew install samba
```
