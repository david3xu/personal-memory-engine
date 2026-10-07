// Resolve durable per-user storage independently of installation and source paths.
use std::{io, path::PathBuf};
pub fn default_data_dir() -> io::Result<PathBuf> {
    dirs::data_local_dir()
        .map(|root| root.join("org.personalmemory.engine"))
        .ok_or_else(|| io::Error::other("Cannot locate per-user application data"))
}
