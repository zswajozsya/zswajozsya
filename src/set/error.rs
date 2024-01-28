use std::fmt::Display;

#[derive(Debug)]
pub enum SetError {
    IO(std::io::Error),
    Ron(ron::error::Error),
}

impl Display for SetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetError::IO(err) => err.fmt(f),
            SetError::Ron(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for SetError {}

impl From<std::io::Error> for SetError {
    fn from(value: std::io::Error) -> Self {
        SetError::IO(value)
    }
}

impl From<ron::error::Error> for SetError {
    fn from(value: ron::error::Error) -> Self {
        SetError::Ron(value)
    }
}
