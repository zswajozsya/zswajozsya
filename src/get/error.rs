use std::fmt::Display;

#[derive(Debug)]
pub enum GetError {
    IO(std::io::Error),
    Spanned(ron::error::SpannedError),
}

impl Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetError::IO(err) => err.fmt(f),
            GetError::Spanned(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for GetError {}

impl From<std::io::Error> for GetError {
    fn from(value: std::io::Error) -> Self {
        GetError::IO(value)
    }
}

impl From<ron::error::SpannedError> for GetError {
    fn from(value: ron::error::SpannedError) -> Self {
        GetError::Spanned(value)
    }
}
