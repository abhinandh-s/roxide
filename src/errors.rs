use std::{io, path::PathBuf};

use thiserror::Error;
use miette::Diagnostic;

#[derive(Error, Debug, Diagnostic)]
pub enum RoxError {
    #[error(transparent)]
    #[diagnostic(code(roxide::io_error))]
    IoError(#[from] io::Error),
    #[error("`{0}` is located on a different device. Can't move item to trash dir.")]
    CrossesDevices(PathBuf),
    #[error("roxide: Directory not empty")]
    DirectoryNotEmpty(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown error")]
    Unknown,
    #[error("roxide: File not found: `{0}`.")]
    NoSuchFile(PathBuf),
    #[error("roxide: failed to remove '{0}': Not a directory")]
    NotADirectory(PathBuf),
    #[error("`{0}` is root! Removal of the root directory is not allowed by design in roxide.")]
    IsRoot(PathBuf),
    #[error("`{0}` is home!")]
    IsHome(PathBuf),
    #[error("roxide: No files found matching the pattern `{0}`.")]
    WriteProtected(PathBuf),
    #[error("Permission denied `{0}` is write-protected")]
    PatternNoMatch(String),
    #[error(transparent)]
    #[diagnostic(code(roxide::any_error))]
    AnyError(#[from] anyhow::Error),
    #[error("Error: can't find cache dir")]
    CantFindCacheDir,
    #[error("Error: can't find config dir")]
    CantFindConfigDir,
}
