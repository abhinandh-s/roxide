use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use anyhow::Error;
use dirs::data_dir;
use serde::{Deserialize, Serialize};

/// # LogId unique id which represents year, month, date, hour, minute and second
/// in this order itself.
///
/// LogId("%Y%m%d%H%M%S")
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogId(u64);

impl From<i32> for LogId {
    fn from(value: i32) -> Self {
        LogId(value as u64)
    }
}
impl From<u64> for LogId {
    fn from(value: u64) -> Self {
        LogId(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrashLog {
    id: LogId,
    from: PathBuf,
    to: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogFile {
    logs: Vec<TrashLog>,
}

pub struct LocalDataDir(pub PathBuf);

impl LocalDataDir {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<PathBuf, Error> {
        if let Some(log_dir) = data_dir() {
            let log_dir = log_dir.join("roxide");
            if !log_dir.exists() {
                create_dir_all(&log_dir)?;
            }
            return Ok(log_dir);
        }
        Err(crate::RoxError::AnyError(anyhow::anyhow!("Failed to get local data dir")).into())
    }
}

pub struct LocalLogFile(pub PathBuf);

impl LocalLogFile {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<PathBuf, Error> {
        if let Some(log_dir) = data_dir() {
            let log_dir = log_dir.join("roxide");
            if !log_dir.exists() {
                create_dir_all(&log_dir)?;
            }
            let log_file = log_dir.join("history.json");
            if !log_file.exists() {
                File::create(&log_file)?;
            }
            return Ok(log_file);
        }
        Err(crate::RoxError::AnyError(anyhow::anyhow!("Failed to get Log file")).into())
    }

    pub fn write(_log: TrashLog) -> Result<(), Error> {
        let log_file = LocalLogFile::new()?;
        let mut _file = std::fs::OpenOptions::new().open(&log_file)?;
        let con = std::fs::read_to_string(log_file)?;
        let json: Vec<TrashLog> = serde_json::from_str(&con)?;

        println!("{:#?}", json);

        // writeln!(file, "{}", log.log_id.0)?;
        // writeln!(file, "{}", log.metadata.file_path.to_string_lossy())?;
        // writeln!(file, "{}", log.metadata.trash_path.to_string_lossy())?;
        // writeln!(file, "----------------------------")?;

        Ok(())
    }
}
// pub fn write(history: History) -> anyhow::Result<(), Error> {
//     let log_file = LocalLogFile::new()?;
//     if !log_file.exists() {
//         File::create(&log_file)?;
//     }
//     let mut file = OpenOptions::new()
//         .create(true) // Create the file if it doesn't exist
//         .append(true) // Append to the file if it already exists
//         .open(log_file)?;
//
//     writeln!(file, "{}", history.log_id.0)?;
//     writeln!(file, "{}", history.metadata.file_path.to_string_lossy())?;
//     writeln!(file, "{}", history.metadata.trash_path.to_string_lossy())?;
//     writeln!(file, "----------------------------")?;
//     Ok(())
// }
//
// #[allow(dead_code, unused)]
// #[allow(clippy::unwrap_used)]
// pub fn init_revert() -> Result<(), Error> {
//     let log_dir = LocalDataDir::new()?;
//     let log_file = log_dir.join("history.log");
//     let file = fs::File::open(&log_file)?;
//     let reader = BufReader::new(file);
//
//     let mut a_vec: Vec<String> = vec::Vec::new();
//     for line in reader.lines() {
//         let line = line?;
//         a_vec.push(line);
//     }
//     Self::revert(
//         a_vec.iter().nth_back(1).unwrap().to_string(),
//         a_vec.iter().nth_back(2).unwrap().to_string(),
//     )?;
//
//     let vec_lng = a_vec.len();
//     debug!("vec len: {}", &vec_lng);
//
//     if vec_lng > 160 {
//         a_vec.truncate(160);
//     }
//     let mut new_file = fs::File::create(log_file)?;
//     for i in a_vec {
//         writeln!(new_file, "{}", i)?;
//     }
//
//     Ok(())
// }
// fn revert(from: String, to: String) -> Result<(), Error> {
//     if !Path::new(&from).exists() {
//         println!("File Doesn't Exist in Trash dir");
//     } else {
//         rename(from, to)?;
//     }
//     Ok(())
// }
// pub fn clean() {}
//
pub trait HistoryOps {
    fn clean();
    fn revert();
    fn write();
}

#[allow(dead_code)]
impl TrashLog {
    pub fn new(id: LogId, from: PathBuf, to: PathBuf) -> Self {
        Self { id, from, to }
    }

    pub fn set_id(&mut self, id: LogId) {
        self.id = id;
    }

    pub fn id(&self) -> &LogId {
        &self.id
    }

    pub fn set_from(&mut self, from: PathBuf) {
        self.from = from;
    }

    pub fn from(&self) -> &PathBuf {
        &self.from
    }

    pub fn set_to(&mut self, to: PathBuf) {
        self.to = to;
    }

    pub fn to(&self) -> &PathBuf {
        &self.to
    }
}
