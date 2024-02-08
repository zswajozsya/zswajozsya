use std::{fs, path::PathBuf};

use crate::{error::Error, Directory};

pub fn set<P: Into<PathBuf>>(path: P, directory: Directory) -> Result<(), Error> {
    let mut path: PathBuf = path.into();
    path.push(".zswajozsya.ron");
    let ron_string = ron::to_string(&directory)?;
    fs::write(path, ron_string)?;
    Ok(())
}
