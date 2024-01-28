use std::path::PathBuf;

type Color = (u8, u8, u8);

struct LabelOption {
    name: String,
    desc: Option<String>,
    color: Color,
}

struct Label {
    name: String,
    desc: Option<String>,
    color: Color,
    options: Vec<LabelOption>,
}

struct File {
    filename: String,
    labels: Vec<usize>,
}

pub struct Directory {
    files: Vec<File>,
    labels: Vec<Label>,
}

pub fn get<P: Into<PathBuf>>(path: P) -> Option<Directory> {
    todo!()
}
pub fn set<P: Into<PathBuf>>(path: P, dir: Directory) {}
pub fn check<P: Into<PathBuf>>(path: P) -> Result<(), ()> {
    todo!()
}
