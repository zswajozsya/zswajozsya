use std::path::PathBuf;

use crate::{error::Error, Directory};

pub fn get<P: Into<PathBuf>>(path: P) -> Result<Directory, Error> {
    let mut path: PathBuf = path.into();
    path.push(".zswajozsya.ron");
    let text = std::fs::read_to_string(path)?;
    let dir = ron::from_str(&text)?;
    Ok(dir)
}
