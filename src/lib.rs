#![deny(
    rust_2018_idioms,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::redundant_clone
)]

mod checks;
pub use checks::*;
mod args;
pub use args::*;
mod remove;
pub use remove::*;
mod config;
mod macros;
pub use config::*;
mod log;
pub use log::*;
mod errors;
pub use errors::*;

/// Splits the given `&Path` into directory path (prefix) and file name (suffix).
///
/// # Arguments
/// - `path`: A referance to `Path` containing the path to be split.
///
/// # Returns
/// - `Ok((String, String))`: A tuple containing the directory path and file name as `String`s
/// - `Err(Box<dyn Error>)`: An error if the delimiter `/` is not found or the path conversion fails. Which means the path only contains file name.
///
/// # Note
/// - It wont check whether the path exists or not
///
/// # Example
///
/// ```
/// use roxide::split_path_and_file;
/// use std::path::Path;
///
/// let i = Path::new("test.txt");
/// match split_path_and_file(&i) {
///     Some((p, s)) => {
///         println!("Got prefix: {p}");
///         println!("Got suffix: {s}");
///         }
///     None => {
///         println!("can't split");
///        }
///     }
/// ```
pub fn split_path_and_file(path: &std::path::Path) -> Option<(String, String)> {
    path.to_str().map(|f| {
        f.rsplit_once("/")
            .map(|(prefix, suffix)| (prefix.to_string(), suffix.to_string()))
    })?
}

pub mod time {
    /// Returns a `DateTime<Local>` which corresponds to the current date and time.
    ///
    /// # Example
    ///
    /// ```
    /// use roxide::time;
    ///
    /// let formatted_time = time::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    /// ```
    pub fn now() -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }
}

///# Returns the path to the user's local trash directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None`.
///
/// |Platform | Value                            | Example                              |
/// | ------- | ---------------------------------| ------------------------------------ |
/// |  Linux  | `$HOME`/.local/share/Trash/files | /home/alice/.local/share/Trash/files |
pub fn trash_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|dir| dir.join("Trash/files"))
}

pub mod filter {
    #![allow(unused_labels)]

    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;

    use anyhow::Result;
    use log::*;
    use walkdir::{DirEntry, WalkDir};

    use crate::{show_error, Cli, RoxError};

    #[derive(Debug)]
    pub struct PathFilter;

    impl PathFilter {
        /// if pattern matches it will return true
        pub fn matches_pattern(args: &Cli, filename: &OsStr) -> bool {
            args.pattern
                .as_deref()
                .map(|pat| filename.to_str().is_some_and(|f| f.contains(pat)))
                .unwrap_or(false)
        }

        fn is_hidden(entry: &DirEntry) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        }

        fn check_exists(item: PathBuf) -> bool {
            item.exists() || {
                // its OR not a colosure
                show_error!(
                    "cannot remove '{}': no such file or directory",
                    &item.display()
                );
                false
            }
        }

        /// # filter_paths
        ///
        /// it will take the arguments [ recursive and pattern ]
        /// and returns items to delete in a vec of PathBuf
        ///
        pub fn filter(items: Vec<PathBuf>, args: &Cli) -> Result<Vec<PathBuf>, anyhow::Error> {
            let mut files: Vec<PathBuf> = Vec::new();
            for item in &items {
                if Self::check_exists(item.into()) {
                    match (args.recursive, args.pattern.is_some()) {
                        (true, true) => {
                            for entry in item {
                                let walker = WalkDir::new(entry).into_iter();
                                for entry in
                                    walker.into_iter().filter_entry(|e| !Self::is_hidden(e))
                                {
                                    let entry = entry?;
                                    if entry.path().is_dir() {
                                        continue;
                                    }
                                    entry
                                        .path()
                                        .file_name()
                                        .map(|filename| {
                                            if Self::matches_pattern(args, filename) {
                                                files.push(entry.path().to_path_buf())
                                            }
                                        })
                                        .ok_or(anyhow::anyhow!("fdd"))?;
                                }
                            }
                        }
                        (true, false) => files.push(item.to_path_buf()),
                        (false, true) => {
                            if item.is_file() {
                                item.file_name()
                                    .map(|filename| {
                                        if Self::matches_pattern(args, filename) {
                                            files.push(item.to_path_buf())
                                        }
                                    })
                                    .ok_or(anyhow::anyhow!("fdd"))?;
                            } else {
                                'iter_dir_contents: for entry in fs::read_dir(item)? {
                                    let entry = entry?;
                                    let path = entry.path();
                                    if path.is_file() {
                                        entry
                                            .path()
                                            .file_name()
                                            .map(|filename| {
                                                if Self::matches_pattern(args, filename) {
                                                    files.push(entry.path())
                                                }
                                            })
                                            .ok_or(anyhow::anyhow!("fdd"))?;
                                    } else {
                                        trace!("skipping directory: {:?}", path);
                                    }
                                }
                            }
                        }
                        (false, false) => {
                            if item.is_file() || args.dir {
                                files.push(item.to_path_buf())
                            } else {
                                show_error!("cannot remove '{}': Is a directory", item.display());
                            }
                        }
                    }
                } else {
                    continue;
                }
            }
            match &args.pattern {
                Some(pat) if files.is_empty() => {
                    return Err(RoxError::PatternNoMatch(pat.to_string()).into());
                }
                _ => {}
            }
            Ok(files)
        }
    }
}
