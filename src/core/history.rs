use chrono::{DateTime, Local};
use roxide::{time, trash_dir, verbose, Cli};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs::{self, create_dir_all, rename, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    num::ParseIntError,
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

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
    pub fn new(file_path: PathBuf, trash_path: PathBuf) -> Self {
        Self {
            file_path,
            trash_path,
        }
    }
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

#[derive(Debug)]
pub struct Trash<'a> {
    pub file: &'a Path,
}

pub trait TrashOps {
    fn exists_in_trash(&self) -> bool
    where
        Self: AsRef<std::path::Path>;
    fn get_unique_id(&self) -> String {
        time::now().format("%Y%m%d%H%M%S").to_string()
    }
    fn get_pretty_unique_id(&self) -> String {
        time::now().format("%Y-%m-%d_%H:%M:%S").to_string()
    }
    fn get_raw_unique_id(&self) -> DateTime<Local> {
        time::now()
    }
}

impl TrashOps for Trash<'_> {
    fn exists_in_trash(&self) -> bool
    where
        Self: AsRef<std::path::Path>,
    {
        trash_dir()
            .unwrap()
            .join(self.file.file_name().unwrap())
            .exists()
    }
}

impl Trash<'_> {
    pub fn get_log_id(&self) -> (String, String) {
        (
            time::now().format("%Y%m%d%H%M%S").to_string(),
            time::now().format("%Y-%m-%d_%H:%M:%S").to_string(),
        )
    }
    pub fn trash_name(&self, log_id: String) -> String {
        let file_stem = self.file.file_stem().unwrap().to_str().unwrap();
        let file_ext = self.file.extension().and_then(|e| e.to_str());
        let trash_file = trash_dir()
            .unwrap()
            .join(self.file.file_name().unwrap())
            .exists();
        let trash_file_name = |stem: &str, ext: Option<&str>| -> String {
            match ext {
                Some(e) => format!("{}.{}.{}", stem, log_id, e),
                None => format!("{}.{}", stem, log_id),
            }
        };
        if !trash_file {
            debug!(
                "impl Trash struct: {:#?}",
                self.file
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap()
            );
            self.file
                .file_name()
                .map(|t| t.to_string_lossy().to_string())
                .expect("failed to set trash name")
        } else {
            let trash_name = trash_file_name(file_stem, file_ext);
            debug!(
                "Trash name from impl: {:#?}",
                self.file
                    .with_file_name(&trash_name)
                    .to_str()
                    .unwrap()
                    .to_string()
            );
            trash_name
        }
    }

    /// This function checks the hash of given file and the file in trash directory.
    ///
    /// if hash matches it will return true.
    pub fn compute_sha256(&self, args: &Cli) -> bool {
        if self.file.is_file() {
            let mut file = File::open(self.file).unwrap();
            let trash_file = trash_dir().unwrap().join(self.file.file_name().unwrap());
            if trash_file.exists() {
                let mut trash_file = File::open(trash_file).unwrap();

                let mut hasher = Sha256::new();
                let mut hasher2 = Sha256::new();

                io::copy(&mut file, &mut hasher).unwrap();
                io::copy(&mut trash_file, &mut hasher2).unwrap();

                let hash = hasher.finalize();
                let hash2 = hasher2.finalize();

                verbose!(
                    args.verbose,
                    "hash of given file and a file in Trash directory matched"
                );
                if hash == hash2 {
                    return true;
                }
            }
        }
        false
    }
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
