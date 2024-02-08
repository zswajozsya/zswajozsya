mod error;
mod get;
mod set;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct LabelOption {
    name: String,
    desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Label {
    name: String,
    desc: String,
    color: String,
    options: Vec<LabelOption>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    name: String,
    labels: Vec<Vec<bool>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Directory {
    entries: Vec<Entry>,
    labels: Vec<Label>,
}

pub use get::get;
pub use set::set;
