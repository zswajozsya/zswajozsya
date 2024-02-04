use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Ron(ron::error::Error),
    Spanned(ron::error::SpannedError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => err.fmt(f),
            Error::Ron(err) => err.fmt(f),
            Error::Spanned(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<ron::error::Error> for Error {
    fn from(value: ron::error::Error) -> Self {
        Error::Ron(value)
    }
}

impl From<ron::error::SpannedError> for Error {
    fn from(value: ron::error::SpannedError) -> Self {
        Error::Spanned(value)
    }
}


