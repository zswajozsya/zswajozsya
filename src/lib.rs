mod error;
mod get;
mod set;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelOption {
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    pub desc: String,
    pub color: String,
    pub options: Vec<LabelOption>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub labels: Vec<Vec<bool>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Directory {
    pub entries: Vec<Entry>,
    pub labels: Vec<Label>,
}

pub use get::get;
pub use set::set;
