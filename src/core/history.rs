use std::error::Error;
use std::fs::{self, create_dir_all, rename, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::vec;

use dirs::data_dir;
use log::debug;

use super::rm::RoError;

/// # LogId unique id which represents year, month, date, hour, minute and second
/// in this order itself. ("%Y%m%d%H%M%S")
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LogId(u64);

impl From<u64> for LogId {
    fn from(value: u64) -> Self {
        LogId(value)
    }
}

impl FromStr for LogId {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(value) => Ok(LogId(value)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TrashMeta {
    pub file_path: PathBuf,
    pub trash_path: PathBuf,
}

impl TrashMeta {
    //    pub fn new(file_path: PathBuf, trash_path: PathBuf) -> Self {
    //        Self { file_path, trash_path }
    //    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct History {
    pub log_id: LogId,
    pub metadata: TrashMeta,
}

impl History {
    pub fn write<'a>(history: History) -> RoError<'a, ()> {
        let log_dir = data_dir().unwrap().join("roxide");
        if !log_dir.exists() {
            create_dir_all(log_dir).unwrap();
        }
        let log_file = data_dir().unwrap().join("roxide/history.log");
        if !log_file.exists() {
            File::create(&log_file).unwrap();
        }
        let mut file = OpenOptions::new()
            .create(true) // Create the file if it doesn't exist
            .append(true) // Append to the file if it already exists
            .open(log_file)?;

        writeln!(file, "{}", history.log_id.0)?;
        writeln!(file, "{}", history.metadata.file_path.to_string_lossy())?;
        writeln!(file, "{}", history.metadata.trash_path.to_string_lossy())?;
        writeln!(file, "----------------------------")?;

        Ok(())
    }

    pub fn init_revert() -> Result<(), Box<dyn Error>> {
        let log_file = data_dir().unwrap().join("roxide/history.log");
        let file = fs::File::open(&log_file)?;
        let reader = BufReader::new(file);

        let mut a_vec: Vec<String> = vec::Vec::new();
        for line in reader.lines() {
            let line = line?;
            a_vec.push(line);
        }
        Self::revert(
            a_vec.iter().nth_back(1).unwrap().to_string(),
            a_vec.iter().nth_back(2).unwrap().to_string(),
        )?;

        let vec_lng = a_vec.len();
        debug!("vec len: {}", &vec_lng);

        if vec_lng > 160 {
            a_vec.truncate(160);
        }
        let mut new_file = fs::File::create(log_file)?;
        for i in a_vec {
            writeln!(new_file, "{}", i)?;
        }

        Ok(())
    }
    fn revert(from: String, to: String) -> Result<(), Box<dyn Error>> {
        if !Path::new(&from).exists() {
            println!("File Doesn't Exist in Trash dir");
        } else {
            rename(from, to)?;
        }
        Ok(())
    }
    pub fn clean() {}
}

#[cfg(test)]
mod test {
    #[test]
    fn get_last_log() {
        #[allow(clippy::useless_vec)]
        let dummy_log = vec![
            "-------------",
            "-------------",
            "-------------",
            "-------------",
            "-------------",
            "20241112214434",
            "/home/abhi/projects/abhi/github/roxide/file004.org",
            "/home/abhi/.local/share/Trash/files/file004.2024-11-12_21:44:34.org",
            "20241112214434",
            "----------------------------",
        ];
        assert_eq!(
            "20241112214434",
            &dummy_log.iter().nth_back(4).unwrap().to_string()
        );
        assert_eq!(
            "/home/abhi/projects/abhi/github/roxide/file004.org",
            &dummy_log.iter().nth_back(3).unwrap().to_string()
        );
        assert_eq!(
            "/home/abhi/.local/share/Trash/files/file004.2024-11-12_21:44:34.org",
            &dummy_log.iter().nth_back(2).unwrap().to_string()
        );
        assert_eq!(
            "20241112214434",
            &dummy_log.iter().nth_back(1).unwrap().to_string()
        );
        assert_eq!(
            "----------------------------",
            &dummy_log.iter().nth_back(0).unwrap().to_string()
        );
    }
}
