mod error;
mod get;
mod init;
mod set;

use serde::{Deserialize, Serialize};

type Color = (u8, u8, u8);

#[derive(Debug, Deserialize, Serialize)]
struct LabelOption {
    name: String,
    desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Label {
    name: String,
    desc: String,
    color: Color,
    options: Vec<LabelOption>,
}

#[derive(Debug, Deserialize, Serialize)]
struct File {
    filename: String,
    labels: Vec<Vec<bool>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Directory {
    files: Vec<File>,
    labels: Vec<Label>,
}

pub use get::get;
pub use init::init;
pub use set::set;
