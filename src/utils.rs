use std::error::Error;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::{DateTime, Local};
use dirs::data_local_dir;
use log::trace;

/// # LogId unique id which represents year, month, date, hour, minute and second
/// in this order itself. ("%Y%m%d%H%M%S")
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LogId {
    pub num: u64,
}

impl From<u64> for LogId {
    fn from(value: u64) -> Self {
        LogId { num: value }
    }
}

impl FromStr for LogId {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(id) => Ok(LogId { num: id }),
            Err(e) => Err(e),
        }
    }
}

///# Returns the path to the user's local trash directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None`.
///
/// |Platform | Value                            | Example                              |
/// | ------- | ---------------------------------| ------------------------------------ |
/// |  Linux  | `$HOME`/.local/share/Trash/files | /home/alice/.local/share/Trash/files |
pub fn trash_dir() -> PathBuf {
    let trash_dir = data_local_dir()
        .expect("Failed to get local data directory")
        .join("Trash/files");
    trace!("trash_dir: {}", &trash_dir.display());
    trash_dir
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
    let c_time = Local::now();
    trace!("{}", &c_time);
    c_time
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

//#[cfg(test)]
//mod tests {
//    use std::fs;
//    use std::path::PathBuf;
//
//    use dirs::data_dir;
//
//    use crate::core::remove_files;
//
//    use super::trash_dir;
//
//    #[test]
//    fn github_test() {
//        if !data_dir().expect("Cant get data dir").join("roxide").exists() {
//            panic!("data dir exists {}", data_dir().unwrap().join("roxide").display());
//        }
//        if trash_dir().exists() {
//            let v0 = PathBuf::from("temp_file_for_single_file_text01.txt");
//            let v1 = PathBuf::from("temp_file_for_single_file_text02.txt");
//            let v2 = PathBuf::from("temp_file_for_single_file_text03.txt");
//            let v3 = PathBuf::from("temp_file_for_single_file_text04.txt");
//            let single_files = vec![v0, v1, v2, v3];
//            for i in &single_files {
//                fs::write(i, "some contents for the files").expect("Cant create files");
//                assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
//            if !i.exists() {
//                    panic!("{} doesnt exists", i.display());
//                }
//            }
//            remove_files(single_files, false, true).expect("Test failed");
//            // panic!("trash dir exists! {}", trash_dir().display());
//        } else {
//            panic!("trash dir doesn't exists");
//        }
//
//
//        // fs::write("file_for_github.txt", "some contents for the files").expect("Cant create files for github");
//        // fs::create_dir_all("some/dir/for/testing").expect("Cant create files for github");
//
//        // remove_files("file_for_github.txt", false, true);
//        // remove_files("some", true, false);
//    }
//}
