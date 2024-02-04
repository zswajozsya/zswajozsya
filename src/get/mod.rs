use std::path::PathBuf;

use crate::{error::Error, Directory};

pub fn get<P: Into<PathBuf>>(path: P) -> Result<Option<Directory>, Error> {
    let mut path: PathBuf = path.into();
    path.push(".zswajozsya.ron");
    if !path.exists() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(path)?;
    let dir = ron::from_str(&text)?;
    Ok(Some(dir))
}
