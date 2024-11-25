use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum Error<'a> {
    NoSuchFile(String),
    IsRoot(&'a Path),
    IsHome(&'a Path),
    // InvalidPattern(String),
    PatternNoMatch(String),
    IoError(std::io::Error),
    // Other(String),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoSuchFile(path) => write!(f, "File not found: {}", path),
            Error::IsRoot(path) => write!(
                f,
                "`{}` is root! Removal of the root directory is not allowed by design in roxide.",
                path.display()
            ),
            Error::IsHome(path) => write!(f, "`{}` is home!", path.display()),
            Error::PatternNoMatch(pat) => {
                write!(f, "roxide: No files found matching the pattern `{}`.", pat)
            }
            Error::IoError(e) => write!(f, "Error: {}", e),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl<'a> From<std::io::Error> for Error<'a> {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}
