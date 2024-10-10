mod args;
mod color;
mod config;
pub mod protocol;

pub mod paths;
use std::fmt::Display;

pub use args::*;
pub use color::*;
pub use config::*;
pub use protocol::*;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    FileParseError(String),
    FFmpegError(String),
    InvalidArgs(String),
    InvalidConfig(String),
    Custom(String),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::FileParseError(e) => write!(f, "File parse error: {}", e),
            Error::FFmpegError(e) => write!(f, "FFmpeg error: {}", e),
            Error::InvalidArgs(e) => write!(f, "Invalid arguments: {}", e),
            Error::InvalidConfig(e) => write!(f, "Invalid config: {}", e),
            Error::Custom(e) => write!(f, "{}", e),
        }
    }
}
