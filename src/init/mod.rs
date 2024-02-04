use std::{fs::DirEntry, path::PathBuf};

use crate::{error::Error, set, Directory, File};

pub fn init<P: Into<PathBuf>>(path: P) -> Result<(), Error> {
    let path: PathBuf = path.into();
    let files = std::fs::read_dir(&path)?
        .map(|e| e)
        .collect::<Result<Vec<DirEntry>, std::io::Error>>()?
        .into_iter()
        .map(|e| File {
            filename: e.file_name().to_string_lossy().to_string(),
            labels: Vec::new(),
        })
        .collect::<Vec<File>>();
    let dir = Directory {
        files: files,
        labels: Vec::new(),
    };
    set(path, dir)?;
    Ok(())
}
