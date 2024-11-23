use std::error::Error;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use dirs::data_local_dir;
use log::trace;


///# Returns the path to the user's local trash directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None`.
///
/// |Platform | Value                            | Example                              |
/// | ------- | ---------------------------------| ------------------------------------ |
/// |  Linux  | `$HOME`/.local/share/Trash/files | /home/alice/.local/share/Trash/files |
pub fn trash_dir() -> PathBuf {
    data_local_dir()
        .expect("Failed to get local data directory")
        .join("Trash/files")
}

/// Returns a `DateTime<Local>` which corresponds to the current date and time.
///
/// # Example
///
/// ```
/// use roxide::utils::current_time;
///
/// let formatted_time = current_time().format("%Y-%m-%d_%H:%M:%S").to_string();
/// ```
pub fn current_time() -> DateTime<Local> {
    Local::now()
}

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
/// use roxide::utils::split_path_and_file;
/// use std::path::Path;
///
/// let i = Path::new("test.txt");
/// match split_path_and_file(&i) {
///     Ok((p, s)) => {
///         println!("Got prefix: {p}");
///         println!("Got suffix: {s}");
///         }
///     Err(_) => {
///         println!("can't split");
///        }
///     }
/// ```
pub fn split_path_and_file(path: &Path) -> Result<(String, String), Box<dyn Error>> {
    match path.to_str().unwrap().rsplit_once("/") {
        Some((prefix, suffix)) => {
            trace!("Prefix: {}", prefix);
            trace!("Sufix: {}", suffix);
            Ok((prefix.to_string(), suffix.to_string()))
        }
        None => {
            log::info!("Delimiter '/' not found in the string.");
            Err("Delimiter '/' not found in the path".into())
        }
    }
}
