use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum Error<'a> {
    NoSuchFile(&'a Path),
    CrossesDevices(&'a Path),
    DirectoryNotEmpty,
    NotADirectory(&'a Path),
    IsRoot(&'a Path),
    IsHome(&'a Path),
    // InvalidPattern(String),
    PatternNoMatch(String),
    IoError(std::io::Error),
    // Other(String),
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CrossesDevices(path) => write!(
                f,
                "`{}` is located on a different device. Can't move item to trash dir.",
                path.display()
            ),
            Error::DirectoryNotEmpty => write!(f, "roxide: Directory not empty"),
            Error::NoSuchFile(path) => {
                write!(f, "roxide: File not found: {}", path.to_string_lossy())
            }
            Error::NotADirectory(path) => write!(
                f,
                "roxide: failed to remove '{}': Not a directory",
                path.to_string_lossy()
            ),
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

impl std::error::Error for Error<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error<'_> {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}
