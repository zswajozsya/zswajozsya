mod get;
mod set;

use serde::{Deserialize, Serialize};

type Color = (u8, u8, u8);

#[derive(Debug, Deserialize, Serialize)]
struct LabelOption {
    name: String,
    desc: Option<String>,
    color: Color,
}

#[derive(Debug, Deserialize, Serialize)]
struct Label {
    name: String,
    desc: Option<String>,
    color: Color,
    options: Vec<LabelOption>,
}

#[derive(Debug, Deserialize, Serialize)]
struct File {
    filename: String,
    labels: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Directory {
    files: Vec<File>,
    labels: Vec<Label>,
}

pub use get::get;
pub use set::set;