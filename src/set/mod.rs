mod error;

use std::{fs, path::PathBuf};

use crate::Directory;
use self::error::SetError;

pub fn set<P: Into<PathBuf>>(path: P, dir: Directory) -> Result<(), SetError> {
    let mut path: PathBuf = path.into();
    path.push(".zswajozsya.ron");
    let ron_string = ron::to_string(&dir)?;
    fs::write(path, ron_string)?;
    Ok(())
}
